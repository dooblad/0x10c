#version 140

uniform sampler2D diffuse_tex;
uniform vec3 color;

in vec3 v_position;
in vec2 v_tex_coord;

out vec4 frag_color;

void main() {
    vec3 diffuse_color = texture(diffuse_tex, v_tex_coord).rgb;
//    vec3 diffuse_color = color;
    frag_color = vec4(diffuse_color, 1.0);
}
