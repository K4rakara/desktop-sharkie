// This is a GPU shader. GPU shaders are really hard to explain, I won't go
// into detail about how they work here.

#version 140

uniform sampler2D tex;

in vec2 v_tex_coords;

out vec4 f_color;

void main() {
	f_color = texture(tex, v_tex_coords);
}

