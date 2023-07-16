#version 330 core

in VS_OUTPUT {
    //vec3 Color;
    vec2 UV;
} IN;

uniform sampler2D planetMap;

out vec4 Color;

void main()
{
    Color = texture(planetMap, IN.UV);
    
    //Color = vec4(0.0f, 1.0f, 0.0f, 1.0f);
}