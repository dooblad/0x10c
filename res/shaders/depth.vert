#version 330 core

uniform mat4 model_matrix;

layout(location = 0) in vec3 v_pos;

void main() {
	gl_Position = model_matrix * vec4(v_pos, 1.0f);
}


