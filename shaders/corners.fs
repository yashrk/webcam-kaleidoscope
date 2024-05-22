#version 100
precision mediump float;
const float PI = 3.1415926;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
  vec4 gradientColor;
  gradientColor.r = uv.x;
  gradientColor.g = uv.y;
  gradientColor.b = smoothstep(0.0, 1.0, 1.0 - sqrt(pow(uv.x, 2.0) + pow(uv.y, 2.0)));
  gradientColor.a = 1.0;
  gl_FragColor = pow(mix(gradientColor, texture2D(Texture, uv), 0.15), vec4(2.0, 2.0, 2.0, 1.0));
}
