#version 330 core

layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_norm;

out vec3 f_pos;
out vec3 f_norm;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

uniform bool reverse_normals;

void main() {
    f_pos = vec3(model * vec4(v_pos, 1.0));
    f_norm = inverse(transpose(mat3(model))) * v_norm;
    gl_Position = projection * view * model * vec4(v_pos, 1.0);
}

