#version 150

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

in vec3 position;

void main() {
    mat4 modelview = view * model;
    gl_Position = projection * modelview * vec4(position, 1.0);
}
