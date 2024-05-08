#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
    gl_FragColor.r = gl_FragColor.r + ((gl_FragCoord.x * gl_FragCoord.x)  / 100000.0) - 0.5;
    gl_FragColor.g = gl_FragColor.g + ((gl_FragCoord.y * gl_FragCoord.y)  / 1000000.0) - 0.5;
    gl_FragColor.b = 1.0 - (gl_FragColor.b + ((gl_FragCoord.y * gl_FragCoord.y)  / 1000000.0));
}
