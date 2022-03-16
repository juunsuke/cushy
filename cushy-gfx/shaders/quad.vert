#version 330 core

// 3x3 transformation matrix, passed as 3 vectors because of reasons
layout (location = 0) in vec3 Mat1;
layout (location = 1) in vec3 Mat2;
layout (location = 2) in vec3 Mat3;

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
// Because sprites are instanced, we can use gl_VertexID%4 to know our relative position
vec2 _pos_mul[4] = vec2[](
	vec2(0.0, 0.0),
	vec2(1.0, 0.0),
	vec2(0.0, 1.0),
	vec2(1.0, 1.0)
);

vec2 get_pos_mul() {
	return _pos_mul[gl_VertexID % 4];
}


void main() {
	// Get the base position of the vertex
	vec2 pos = get_pos_mul() * Size;

	// Build the 3x3 model transformation matrix
	mat3 model = mat3(Mat1, Mat2, Mat3);

	// Apply it to the calculated position
	vec3 mpos = model * vec3(pos, 1.0);

	// Calculate the final position using the projection matrix
	gl_Position = UniProj * vec4(mpos, 1.0);

	// Pass-through color and textures
	Output.color = Color;
	Output.tex_coord = Tex + (TexDelta * get_pos_mul());
}

