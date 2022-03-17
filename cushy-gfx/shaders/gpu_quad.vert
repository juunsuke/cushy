#version 330 core

// Translation/Rotation/Scale/Origin
layout (location = 0) in vec2 Trans;
layout (location = 1) in float Rot;
layout (location = 2) in vec2 Scale;
layout (location = 3) in vec2 Origin;

// Pos
layout (location = 4) in vec2 Pos;

// Color/alpha tint
layout (location = 5) in vec4 Color;

// Texture coordinates
layout (location = 6) in vec2 Tex;

// Output for the fragment shader
out VS_OUTPUT {
	vec4 color;
	vec2 tex_coord;
} Output;

// The projection matrix
uniform mat4 UniProj;


mat3 translate(in vec2 v)
{
	return mat3(
		1, 0, 0,
		0, 1, 0,
		v.x, v.y, 1
	);
}

mat3 rotate(in float angle)
{
	float c = cos(angle);
	float s = sin(angle);

	return mat3(
		c,  s, 0,
		-s, c, 0,
		0,  0, 1
	);
}

mat3 scale(in vec2 v)
{
	return mat3(
		v.x,	0,		0,
		0,		v.y,	0,
		0,		0,		1
	);
}

void main() {
	// Build the model matrix
	mat3 model = translate(Trans) * rotate(Rot) * scale(Scale) * translate(-Origin);

	// Transform the position
	vec3 mpos = model * vec3(Pos, 1.0);

	// Calculate the final position using the projection matrix
	gl_Position = UniProj * vec4(mpos, 1.0);

	// Pass-through color and textures
	Output.color = Color;
	Output.tex_coord = Tex;
}

