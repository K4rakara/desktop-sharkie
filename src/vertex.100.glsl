// This is a GPU shader. GPU shaders are really hard to explain, I won't go
// into detail about how they work here.

#version 100

uniform lowp mat4 matrix;

attribute lowp vec2 position;
attribute lowp vec2 tex_coords;

varying lowp vec2 v_tex_coords;

void main() {
	gl_Position = matrix * vec4(position, 0.0, 1.0);
	v_tex_coords = tex_coords;
}

