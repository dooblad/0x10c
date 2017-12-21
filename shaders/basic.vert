#version 150

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

in vec3 position;
in vec3 normal;
in vec2 tex_coord;

out vec3 world_pos;
out vec3 world_norm;
out vec3 world_eye;
out vec2 frag_tex_coord;

void main() {
    mat4 modelview_matrix = view_matrix * model_matrix;
    // Use the inverse transpose to preserve normal directions in the presence of
    // non-uniform scaling.
    mat3 normal_matrix = transpose(inverse(mat3(model_matrix)));

    // Calculate some of the values for lighting in the fragment shader.
    world_pos = vec3(model_matrix * vec4(position, 1.0));
    world_norm = normalize(normal_matrix * normal);
    world_eye = vec3(inverse(view_matrix) * vec4(0.0, 0.0, 0.0, 1.0));

    // Interpolate UV.
    frag_tex_coord = tex_coord;

    gl_Position = projection_matrix * modelview_matrix * vec4(position, 1.0);
}
