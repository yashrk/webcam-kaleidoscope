#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;

const int range = 9;
const float sample_sq = pow(float((range*2)+1), 2.0);
const float weight = 0.3;

void main() {
  vec4 env = vec4(0.0, 0.0, 0.0, 1.0);
  for (int x = -range; x<range; x++) {
    for (int y = -range; y<range; y++) {
      env += texture2D(Texture, uv+vec2(x, y));
    }
  }
  env = (env / sample_sq) * weight;
  gl_FragColor = texture2D(Texture, uv) + env;
  gl_FragColor.a = 1.0;
}
