#version 100
const float PI = 3.14159265358;
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;
uniform float short_cycle;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    float rotation = short_cycle / 10000.0 * 2. * PI;
    mat2 rotation_matrix = mat2(cos(rotation), -sin(rotation),
                                sin(rotation), cos(rotation));
    uv = texcoord;
    uv -= 0.5; // Center it
    uv = uv * rotation_matrix;
    uv += 0.5; // Back from centered form
}