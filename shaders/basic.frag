#version 150

// Material Properties
uniform vec3 diffuse_color;
// Light Properties
uniform vec3 light_position;

in vec3 world_pos;
in vec3 world_norm;
in vec3 world_eye;

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
    float shininess = 20.0;
    float specular_shade = B * pow(max(dot(H, N), 0.0), shininess);

    float r = length(light_position - world_pos);

    float fa_denom = 0.5 + r * 0.1 + r * r * 0.1;
    if (fa_denom == 0.0) {
        fa_denom = 0.01;
    }
    float fa = 1.0 / fa_denom;

    vec3 ambient = diffuse_color * vec3(0.6, 0.2, 0.2);
    vec3 diffuse = diffuse_shade * diffuse_color * vec3(0.6, 0.2, 0.2) * fa;
    vec3 specular = specular_shade * vec3(1.0, 1.0, 1.0) * vec3(0.6, 0.2, 0.2) * fa;

    //frag_color = vec4(diffuse_color, 1.0);
    frag_color = vec4(ambient + diffuse + specular, 1.0);
}
