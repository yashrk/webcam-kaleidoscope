#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
    gl_FragColor.r = pow(gl_FragColor.r,3.0);
    gl_FragColor.g = pow(gl_FragColor.g,3.0);
    gl_FragColor.b = pow(gl_FragColor.b,3.0);
}
