#version 330 core

// 3x3 transformation matrix
layout (location = 0) in mat3 Mat;

// Sprite size, in pixels (width x height)
layout (location = 3) in vec2 Size;

// Color/alpha tint
layout (location = 4) in vec4 Color;

// Coordinates for (u1, v1) along with (du, dv) delta
// (u2, v2) = Tex+TexDelta
layout (location = 5) in vec2 Tex;
layout (location = 6) in vec2 TexDelta;

// Output for the fragment shader
out VS_OUTPUT {
	vec4 color;
	vec2 tex_coord;
} Output;

// The projection matrix
uniform mat4 UniProj;

// Position multiplier
// Because quads are instanced, we can use gl_VertexID%6 to know our relative position
// (Each quad is made up of 2 triangles, thus 6 vertices.  However, only 1 vertex is needed in the VB)
//
// 0   1         3
// 2          4  5
//
vec2 _pos_mul[6] = vec2[](
	vec2(0.0, 0.0),
	vec2(1.0, 0.0),
	vec2(0.0, 1.0),
	vec2(1.0, 0.0),
	vec2(0.0, 1.0),
	vec2(1.0, 1.0)
);

vec2 get_pos_mul() {
	return _pos_mul[gl_VertexID % 6];
}


void main() {
	// Get the base position of the vertex
	vec2 pos = get_pos_mul() * Size;

	// Apply it to the calculated position
	vec3 mpos = Mat * vec3(pos, 1.0);

	// Calculate the final position using the projection matrix
	gl_Position = UniProj * vec4(mpos, 1.0);

	// Pass-through color and textures
	Output.color = Color;
	Output.tex_coord = Tex + (TexDelta * get_pos_mul());
}

