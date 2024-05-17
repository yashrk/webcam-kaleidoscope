#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform float ms_time;
uniform float short_cycle;

void main() {
  gl_FragColor = texture2D(Texture, uv);
  gl_FragColor.r = gl_FragColor.r * abs(sin(short_cycle / 1000.0));
  gl_FragColor.g = gl_FragColor.g * uv.y * 0.5;
  gl_FragColor.b = gl_FragColor.b * pow((1.0 - (uv.x + uv.y)), 2.0);
  gl_FragColor.a = 1.0;
}