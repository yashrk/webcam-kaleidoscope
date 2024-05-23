#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;

float peak(float center, float slope, float height) {
  return (-pow(uv.x-center, 2.0)+height)*slope;
}

void main() {
  vec4 textureColor = texture2D(Texture, uv);
  vec4 rainbow;
  rainbow.r = max(peak(-0.5,0.8,1.2),peak(1.2,1.2,0.3));
  rainbow.g = peak(0.5,3.2,0.5);
  rainbow.b = peak(0.9,0.7,1.2);
  rainbow.a = 1.0;
  gl_FragColor = textureColor * rainbow;
}
