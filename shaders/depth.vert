#version 330 core

uniform mat4 model;

layout(location = 0) in vec3 v_pos;

void main() {
	gl_Position = model * vec4(v_pos, 1.0f);
}


