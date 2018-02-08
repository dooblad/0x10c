#version 330 core

uniform samplerCube depth_map;

in vec4 world_pos;
in vec2 frag_tex_coord;

out vec4 frag_color;

void main() {
    float depth = texture(depth_map, normalize(world_pos.xyz)).r;
    frag_color = vec4(depth, depth, depth, 1.0);
}
