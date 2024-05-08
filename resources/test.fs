#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
    gl_FragColor.r = gl_FragColor.g * (1.0 - gl_FragColor.b) * 3.0;
}
