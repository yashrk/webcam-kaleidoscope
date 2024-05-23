#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;

float peak(float center, float slope, float height) {
  return height/(pow(slope * uv.x-center, 2.0) + 1.);
}

void main() {
  vec4 textureColor = texture2D(Texture, uv);
  float meanColor = (textureColor.r + textureColor.g + textureColor.b)/3.0;
  vec4 rainbow;
  rainbow.r = max(peak(0.,1.5,1.),peak(2.1,2.5,0.5));
  rainbow.g = peak(1.0,2.5,1.);
  rainbow.b = peak(1.8,3.,1.3);
  rainbow.a = 1.0;
  gl_FragColor = vec4(meanColor, meanColor, meanColor, 1.0) * rainbow;
}
