#version 460 core
in vec2 TexCoord;
out vec4 FragColor;
layout (location = 7) uniform sampler2D texture1;
void main() {
    vec2 flippedCoord = vec2(TexCoord.x, 1.0 - TexCoord.y);
    FragColor = texture(texture1, flippedCoord);
}