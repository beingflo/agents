#version 330

in vec2 position;

uniform mat4 perspective_zoom;
uniform mat4 perspective_shift;
uniform mat4 model;

void main() {
    gl_Position = perspective_shift * perspective_zoom * model * vec4(position.x, position.y, 0.0, 1.0);
}
