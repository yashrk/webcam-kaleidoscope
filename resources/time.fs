#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;
uniform float ms_time;

void main() {
  gl_FragColor = texture2D(Texture, uv);
  gl_FragColor.r = gl_FragColor.r * abs(sin(ms_time * 0.0002));
  gl_FragColor.g = gl_FragColor.g * abs(sin(ms_time * 0.0003));
  gl_FragColor.b = gl_FragColor.b * (1.0 - abs(sin(ms_time * 0.0005)));
}
