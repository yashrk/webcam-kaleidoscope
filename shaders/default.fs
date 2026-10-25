#version 100
precision mediump float;
const float PI = 3.1416;

varying vec2 uv;

uniform sampler2D Texture;
uniform float phase;

void main() {
    gl_FragColor = texture2D(Texture, uv);
    // Colors always shimmer, whatever the effect (was time.fs)
    gl_FragColor.rgb *= vec3(sin(phase * 12. * PI + 0.2), sin(phase * 18. * PI + 0.79), cos(phase * 6. * PI + 1.7)) * 0.5 + 0.5;
}
