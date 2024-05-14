#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;
uniform float short_cycle;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
    uv.x = uv.x + sin(short_cycle / 10.0) * 0.01;
    uv.y = uv.y + cos(short_cycle / 10.0) * 0.01;
}
