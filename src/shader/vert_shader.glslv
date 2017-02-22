#version 330

in vec2 position;

uniform mat4 perspective;
uniform mat4 model;

void main() {
    gl_Position = perspective * model * vec4(position.x, position.y, 0.0, 1.0);
}
