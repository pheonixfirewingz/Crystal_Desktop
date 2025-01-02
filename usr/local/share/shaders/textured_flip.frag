#version 460 core
layout (location = 0) in VS_OUT {
    vec2 TexCoords;
} fs_in;
layout (location = 0) out vec4 FragColor;
layout (location = 3) uniform sampler2D texture1;
void main() {
    vec2 flippedCoord = vec2(fs_in.TexCoords.x, 1.0 - fs_in.TexCoords.y);
    FragColor = texture(texture1, flippedCoord);
}