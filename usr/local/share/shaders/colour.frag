// Fragment shader
#version 460 core

layout (location = 0) in VS_OUT {
    vec2 TexCoords;
} fs_in;

layout (location = 0) out vec4 FragColor;
layout (location = 3) uniform vec4 uColor;         // Base color of the quad
layout (location = 4) uniform vec2 uResolution;    // Window resolution

float roundedBoxSDF(vec2 centerPosition, vec2 size, float radius) {
    vec2 q = abs(centerPosition) - size + radius;
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - radius;
}

void main()
{
    // Convert texture coordinates to pixel coordinates
    vec2 pixelPos = fs_in.TexCoords * uResolution;

    // Center the coordinate system
    vec2 centerPos = pixelPos - (uResolution * 0.5);

    // Calculate the distance from the rounded rectangle
    float distance = roundedBoxSDF(centerPos, uResolution * 0.5, 24);

    // Create smooth edges
    float smoothness = 1.0;
    float alpha = 1.0 - smoothstep(-smoothness, 0.0, distance);

    // Output final color with transparency
    FragColor = vec4(uColor.rgb, uColor.a * alpha);
}