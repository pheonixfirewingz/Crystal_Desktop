#version 460 core
layout (location = 0) uniform mat4 model;
layout (location = 1) uniform mat4 view;
layout (location = 2) uniform mat4 proj;

out vec2 TexCoord;

const int index[6] = {
0, 1, 2, 2, 3, 0
};

const vec3 vertices[4] = {
// x, y, z
vec3(0.0, 0.0, 0.0), // bottom-left (origin)
vec3(1.0, 0.0, 0.0), // bottom-right
vec3(1.0, 1.0, 0.0),// top-right
vec3(0.0, 1.0, 0.0)// top-left
};

const vec2 uvs[4] = {
// x, y, z
vec2(0.0, 0.0), // bottom-left (origin)
vec2(1.0, 0.0), // bottom-right
vec2(1.0, 1.0),// top-right
vec2(0.0, 1.0)// top-left
};
void main() {
    gl_Position = proj * view * model * vec4(vertices[index[gl_VertexID]], 1.0);
    TexCoord = uvs[index[gl_VertexID]];
}