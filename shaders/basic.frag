#version 150

// Material Properties
uniform vec3 diffuse_color;
uniform bool use_texture;
uniform sampler2D diffuse_texture;
const float shininess = 20.0;
const vec3 emissive_color = vec3(0.0, 0.0, 0.0);
const vec3 specular_color = vec3(1.0, 1.0, 1.0);

// Light Properties
uniform vec3 light_position;
const vec3 light_ambient = vec3(0.05, 0.05, 0.15);
const vec3 light_diffuse = vec3(0.1, 0.1, 0.6);
// Order: [const, linear, quadratic]
const vec3 light_atten_coeffs = vec3(0.01, 0.01, 0.01);

in vec3 world_pos;
in vec3 world_norm;
in vec3 world_eye;
in vec2 frag_tex_coord;

out vec4 frag_color;

void main() {
    vec3 N = world_norm;
    // Viewing direction
    vec3 V = normalize(world_eye - world_pos);
    // Light direction
    vec3 L = normalize(light_position - world_pos);
    // Halfway vector
    vec3 H = normalize(V + L);

    float B = 1.0;
    if (dot(N, L) < 0.00001) { B = 0.0; }

    float diffuse_shade = max(dot(N, L), 0.0);
    float specular_shade = B * pow(max(dot(H, N), 0.0), shininess);

    float r = length(light_position - world_pos);

    float fa_denom = light_atten_coeffs[0];
    fa_denom += r * light_atten_coeffs[1];
    fa_denom += r * r * light_atten_coeffs[2];
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
    vec3 specular = specular_shade * vec3(1.0, 1.0, 1.0) * vec3(0.1, 0.1, 0.6) * fa;

    frag_color = vec4(emissive_color, 1.0) + vec4(ambient + diffuse + specular, 1.0);
}
