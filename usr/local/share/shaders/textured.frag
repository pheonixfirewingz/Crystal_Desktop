#version 460 core
layout (location = 0) in VS_OUT {
    vec2 TexCoords;
} fs_in;
layout (location = 0) out vec4 FragColor;

layout (location = 3) uniform sampler2D texture1;
void main() {
    vec4 texColor = texture(texture1, fs_in.TexCoords);
    FragColor = texColor;
}