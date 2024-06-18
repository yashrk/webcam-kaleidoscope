#version 100
precision mediump float;
const float PI = 3.1416;


varying vec2 uv;

uniform sampler2D Texture;
uniform float phase;

void main() {
  gl_FragColor = texture2D(Texture, uv);
  gl_FragColor.r = gl_FragColor.r * ((sin(phase * 12. * PI + 0.2)) * 0.5 + 0.5);
  gl_FragColor.g = gl_FragColor.g * ((sin(phase * 18. * PI + 0.79)) * 0.5 + 0.5);
  gl_FragColor.b = gl_FragColor.b * ((cos(phase * 6. * PI + 1.7)) * 0.5 + 0.5);
}
