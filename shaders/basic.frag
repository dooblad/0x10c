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

// Shadow-Related Uniforms
uniform samplerCube depth_map;
uniform float far_plane;

in vec3 world_pos;
in vec3 world_norm;
in vec2 frag_tex_coord;

out vec4 frag_color;

float shadow_factor(vec3 frag_pos) {
    vec3 frag_to_light = frag_pos - light_position;
    float current_depth = length(frag_to_light);
    if (current_depth > far_plane) {
        // Out of range of cube map.  In this case, default to no shadow.
        return 0.0;
    }
    float bias = 0.1;
    float closest_depth = texture(depth_map, frag_to_light).r;
    closest_depth *= far_plane;
    if (current_depth - bias > closest_depth) {
        // In shadow
        return 1.0;
    } else {
        // Not in shadow
        return 0.0;
    }
}

void main() {
    vec3 N = world_norm;
    // Light Direction
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

    float shadow = shadow_factor(world_pos);

    vec3 lighting = emissive_color + (ambient + (1.0 - shadow) * diffuse);

    frag_color = vec4(lighting, 1.0);
}
