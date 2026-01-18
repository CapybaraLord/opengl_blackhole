#version 330 core

uniform float u_time;

in vec3 vColor;

void main() {
  float time = sin(u_time);
  gl_FragColor = vec4(vColor, 1.0);
}
