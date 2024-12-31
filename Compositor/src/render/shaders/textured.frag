#version 460 core
in vec2 TexCoord;
out vec4 FragColor;
layout (location = 7) uniform sampler2D texture1;
void main() {
    vec4 texColor = texture(texture1, TexCoord);
    FragColor = texColor;
}