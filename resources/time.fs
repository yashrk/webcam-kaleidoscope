#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;
uniform float uTime;

void main() {
  gl_FragColor = texture2D(Texture, uv);
  gl_FragColor.r = abs(sin(u_time));
}
