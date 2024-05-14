#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;
uniform float short_cycle;

void main() {
  gl_FragColor = texture2D(Texture, uv);
  gl_FragColor.r = gl_FragColor.r * ((sin(short_cycle * 0.0002)) * 0.5 + 0.5);
  gl_FragColor.g = gl_FragColor.g * ((sin(short_cycle * 0.003)) * 0.5 + 0.5);
  gl_FragColor.b = gl_FragColor.b * ((cos(short_cycle * 0.0005)) * 0.5 + 0.5);
}
