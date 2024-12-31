// Vertex Shader
#version 460 core
layout (location = 0) uniform mat4 model;
layout (location = 1) uniform mat4 view;
layout (location = 2) uniform mat4 proj;

out gl_PerVertex {
    vec4 gl_Position;
};

out VS_OUT {
    vec2 position;
} vs_out;

const int index[6] = {
0, 1, 2, 2, 3, 0
};

const vec2 vertices[4] = {
vec2(0.0, 0.0), // bottom-left (origin)
vec2(1.0, 0.0), // bottom-right
vec2(1.0, 1.0), // top-right
vec2(0.0, 1.0)  // top-left
};

void main() {
    vec2 pos = vertices[index[gl_VertexID]];
    gl_Position = proj * view * model * vec4(pos, 0.0, 1.0);
    vs_out.position = (gl_Position.xyz / gl_Position.w).xy;
}