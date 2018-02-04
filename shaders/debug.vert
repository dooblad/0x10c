#version 330 core

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

in vec3 position;
in vec2 tex_coord;

out vec4 world_pos;
out vec2 frag_tex_coord;

void main() {
    // Do no transformations, so we leave the cube at the origin.
    world_pos = model_matrix * vec4(position, 1.0);

    // Interpolate UV coords.
    frag_tex_coord = tex_coord;

    gl_Position = projection_matrix * view_matrix * model_matrix * world_pos;
}
