#version 330

in vec2 position;

uniform float pos_x;
uniform float pos_y;
uniform float r;


void main() {
    float x_scale = position.x / 10;
    float y_scale = position.y / 10;

    float x_final = x_scale - pos_x;
    float y_final = y_scale - pos_y;
    gl_Position = vec4(x_final, y_final, 0.0, 1.0);
}
