#version 330 core

// Material Properties
uniform vec3 diffuse_color;
uniform bool use_texture;
uniform sampler2D diffuse_texture;
const vec3 emissive_color = vec3(0.0, 0.0, 0.0);

// Light Properties
uniform vec3 light_position;
const vec3 light_ambient = vec3(0.1, 0.1, 0.25);
const vec3 light_diffuse = vec3(0.3, 0.3, 1.0);
// Order: [const, linear, quadratic]
const vec3 light_atten_coeffs = vec3(0.01, 0.1, 0.01);

in vec3 world_pos;
in vec3 world_norm;
in vec2 frag_tex_coord;

out vec4 frag_color;

void main() {
    vec3 N = world_norm;
    // Light direction
    vec3 L = normalize(light_position - world_pos);

    float diffuse_shade = max(dot(N, L), 0.0);

    float r = length(light_position - world_pos);

    float fa_denom = light_atten_coeffs[0];
    fa_denom += r * light_atten_coeffs[1];
    // fa_denom += r * r * light_atten_coeffs[2];
    if (fa_denom == 0.0) {
        fa_denom = 0.01;
    }
    float fa = 1.0 / fa_denom;

    vec3 diffuse_component;
    if (use_texture) {
        // Sample texture if it's bound.
        diffuse_component = texture(diffuse_texture, frag_tex_coord).rgb;
    } else {
        diffuse_component = diffuse_color;
    }

    vec3 ambient = diffuse_component * light_ambient;
    vec3 diffuse = diffuse_shade * diffuse_component * light_diffuse * fa;

    // frag_color = vec4(emissive_color, 1.0) + vec4(ambient + diffuse, 1.0);
    frag_color = vec4(diffuse_component, 1.0);
}
