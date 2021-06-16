#version 100

uniform lowp mat4 matrix;

attribute lowp vec2 position;
attribute lowp vec2 tex_coords;

varying lowp vec2 v_tex_coords;

void main() {
	gl_Position = matrix * vec4(position, 0.0, 1.0);
	v_tex_coords = tex_coords;
}

