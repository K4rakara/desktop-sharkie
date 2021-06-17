// This is a GPU shader. GPU shaders are really hard to explain, I won't go
// into detail about how they work here.

#version 110

uniform sampler2D tex;

varying vec2 v_tex_coords;

void main() {
	gl_FragColor = texture2D(tex, v_tex_coords);
}

