#version 150

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

in vec3 position;
in vec2 tex_coord;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coord;

void main() {
    v_tex_coord = tex_coord;
    mat4 modelview = view * model;
    gl_Position = projection * modelview * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}
