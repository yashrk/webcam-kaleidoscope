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
    float rotation = phase * 6. * PI;
    mat2 rotation_matrix = mat2(cos(rotation), -sin(rotation),
                                sin(rotation), cos(rotation));
    uv = texcoord;
    uv -= 0.5; // Center it
    uv = uv * rotation_matrix;
    uv += 0.5; // Back from centered form
}
