#version 330 core

// Input from fragment shader
in VS_OUTPUT {
	vec4 color;
	vec2 tex_coord;
} Input;

// The bound 2D texture
uniform sampler2D UniTexture;

out vec4 Output;

void main() {
	Output = texture2D(UniTexture, Input.tex_coord) * Input.color;
	//Output = Input.color;
}


