#version 100

uniform lowp sampler2D tex;

varying lowp vec2 v_tex_coords;

void main() {
	gl_FragColor = texture2D(tex, v_tex_coords);
}

