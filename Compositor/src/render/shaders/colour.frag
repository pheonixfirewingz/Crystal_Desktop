#version 460 core
layout (location = 0) out vec4 fragColor;
layout (location = 3) uniform vec2 viewportSize;
layout (location = 4) uniform vec4 colour;
layout (location = 5) uniform vec2 quadSize;
layout (location = 6) uniform float radius;
in VS_OUT {
    vec2 position;
} fs_in;

void main() {
    if(radius == 0.0)
    {
        fragColor = colour;
        return;
    }
    else
    {
        fragColor = colour;
    }
}