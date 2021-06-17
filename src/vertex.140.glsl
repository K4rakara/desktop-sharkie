// This is a GPU shader. GPU shaders are really hard to explain, I won't go
// into detail about how they work here.

#version 140

uniform mat4 matrix;

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

void main() {
	gl_Position = matrix * vec4(position, 0.0, 1.0);
	v_tex_coords = tex_coords;
}

