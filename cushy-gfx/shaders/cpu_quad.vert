#version 330 core

// Pre-transformed vertex
layout (location = 0) in vec3 Vert;

// Color/alpha tint
layout (location = 1) in vec4 Color;

// Texture coordinates
layout (location = 2) in vec2 Tex;

// Output for the fragment shader
out VS_OUTPUT {
	vec4 color;
	vec2 tex_coord;
} Output;

// The projection matrix
uniform mat4 UniProj;


void main() {
	// Calculate the final position using the projection matrix
	gl_Position = UniProj * vec4(Vert, 1.0);

	// Pass-through color and textures
	Output.color = Color;
	Output.tex_coord = Tex;
}

