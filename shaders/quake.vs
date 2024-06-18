#version 100
precision mediump float;
const float PI = 3.1416;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;
uniform float phase;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
    uv.x = uv.x + sin(phase * PI * 1000.) * 0.01;
    uv.y = uv.y + cos(phase * PI * 1000.) * 0.01;
}
