#version 330 core

layout (triangles) in;
layout (triangle_strip, max_vertices=18) out;

uniform mat4 depth_transforms[6];

// Fragment position from geometry shader (output per EmitVertex call).
out vec4 frag_pos;

void main() {
    for (int face = 0; face < 6; ++face) {
        // Built-in variable that specifies which face we render to.
        gl_Layer = face;
        // One loop for each vertex of a triangle.
        for (int i = 0; i < 3; ++i) {
            frag_pos = gl_in[i].gl_Position;
            gl_Position = depth_transforms[face] * frag_pos;
            EmitVertex();
        }
        EndPrimitive();
    }
}

