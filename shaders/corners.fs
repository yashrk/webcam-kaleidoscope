#version 100
precision mediump float;
const float PI = 3.1416;

varying vec2 uv;

uniform sampler2D Texture;
uniform float phase;

void main() {
  vec4 gradientColor;
  gradientColor.r = uv.x;
  gradientColor.g = uv.y;
  gradientColor.b = smoothstep(0.0, 1.0, 1.0 - sqrt(pow(uv.x, 2.0) + pow(uv.y, 2.0)));
  gradientColor.a = 1.0;
  gl_FragColor = gradientColor * texture2D(Texture, uv);
  // Colors always shimmer, whatever the effect (was time.fs)
  gl_FragColor.rgb *= vec3(sin(phase * 12. * PI + 0.2), sin(phase * 18. * PI + 0.79), cos(phase * 6. * PI + 1.7)) * 0.5 + 0.5;
}
