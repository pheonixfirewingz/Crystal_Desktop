#version 150 core

uniform sampler2D texSampler;

in vec2 fragTexCoord;

out vec4 outColor;

void main() {
    outColor = texture(texSampler, fragTexCoord);
}
