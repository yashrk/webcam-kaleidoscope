#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;

float peak(float center, float slope, float height) {
  return (-pow(uv.x-center, 2.0)+height)*slope;
}

void main() {
  vec4 textureColor = texture2D(Texture, uv);
  float meanColor = (textureColor.r + textureColor.g + textureColor.b)/3.0;
  vec4 rainbow;
  rainbow.r = max(peak(-0.2,0.7,2.0),peak(1.2,1.2,0.3));
  rainbow.g = peak(0.65,3.2,0.45);
  rainbow.b = peak(1.1,0.9,1.3);
  rainbow.a = 1.0;
  gl_FragColor = vec4(meanColor, meanColor, meanColor, 1.0) * rainbow;
}
