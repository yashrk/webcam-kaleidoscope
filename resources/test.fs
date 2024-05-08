#version 100
precision lowp float;

varying vec2 uv;
uniform vec2 u_resolution;

uniform sampler2D Texture;

void main() {
    vec2 st = gl_FragCoord.xy/u_resolution;
    gl_FragColor = texture2D(Texture, uv);
    gl_FragColor.r = (gl_FragColor.r + (pow((gl_FragCoord.y - 700.0),2.0)+pow((gl_FragCoord.x - 800.0),2.0))  / 200000.0) * 0.7;
    gl_FragColor.g = (gl_FragColor.g - (gl_FragCoord.y / 1000.0)) * 5.0;
    gl_FragColor.b = (gl_FragColor.b - (pow((gl_FragCoord.y - 700.0),2.0)+pow((gl_FragCoord.x - 800.0),2.0))  / 200000.0) * 3.0;
}
