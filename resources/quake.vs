#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;
uniform float ms_time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
    uv.x = uv.x + sin(ms_time) * 0.01;
    uv.y = uv.y + cos(ms_time) * 0.01;
}
