#version 100
precision mediump float;
const float PI = 3.1416;

varying vec2 uv;

uniform sampler2D Texture;
uniform float phase;

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
  // Colors always shimmer, whatever the effect (was time.fs)
  gl_FragColor.rgb *= vec3(sin(phase * 12. * PI + 0.2), sin(phase * 18. * PI + 0.79), cos(phase * 6. * PI + 1.7)) * 0.5 + 0.5;
}
