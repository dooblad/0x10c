#version 330 core
out vec4 frag_color;

in vec3 f_pos;
in vec3 f_norm;

uniform samplerCube depth_map;

uniform vec3 light_pos;
uniform vec3 view_pos;

uniform float far_plane;

float shadow_calculation(vec3 frag_pos) {
    vec3 frag_to_light = frag_pos - light_pos;
    float current_depth = length(frag_to_light);
    float view_distance = length(view_pos - frag_pos);
    float bias = 0.15;
    float closest_depth = texture(depth_map, frag_to_light).r;
    closest_depth *= far_plane;
    if (current_depth - bias > closest_depth) {
        return 1.0f;
    } else {
        return 0.0f;
    }
}

void main() {
    vec3 color = vec3(0.7f, 0.2f, 0.2f);
    vec3 normal = normalize(f_norm);
    vec3 lightColor = vec3(0.2f, 0.2f, 0.7f);

    // ambient
    vec3 ambient = 0.3 * color;
    // diffuse
    vec3 light_dir = normalize(light_pos - f_pos);
    float diff = max(dot(light_dir, normal), 0.0);
    vec3 diffuse = diff * lightColor;
    // specular
    vec3 view_dir = normalize(view_pos - f_pos);
    vec3 reflect_dir = reflect(-light_dir, normal);
    // calculate shadow
    float shadow = shadow_calculation(f_pos);
    vec3 lighting = (ambient + (1.0 - shadow) * diffuse) * color;

    frag_color = vec4(lighting, 1.0);
}

