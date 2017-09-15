#version 330

uniform mat4 matrix;

layout(triangles) in;
layout(triangle_strip, max_vertices=3) out;

out vec3 color;

float rand(vec2 co) {
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

void main() {
    vec3 all_color = vec3(
        rand(gl_in[0].gl_Position.xy + gl_in[1].gl_Position.yz),
        rand(gl_in[1].gl_Position.yx + gl_in[2].gl_Position.zx),
        rand(gl_in[0].gl_Position.xz + gl_in[2].gl_Position.zy)
    );

    gl_Position = matrix * gl_in[0].gl_Position;
    color = all_color;
    EmitVertex();

    gl_Position = matrix * gl_in[1].gl_Position;
    color = all_color;
    EmitVertex();

    gl_Position = matrix * gl_in[2].gl_Position;
    color = all_color;
    EmitVertex();
}

