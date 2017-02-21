#version 330

in vec2 position;

uniform float pos_x;
uniform float pos_y;
uniform float r;


void main() {
    gl_Position = vec4(position.x - pos_x, position.y - pos_y, 0.0, 1.0);
}
