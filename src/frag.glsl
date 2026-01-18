#version 330 core

uniform float u_time;

in vec3 vColor;
in vec2 vUV;

void main() {
  vec2 p = vUV * 12.0;
  float check = mod(floor(p.x) + floor(p.y), 2.0);
  vec3 c = mix(vec3(1.0, 1.0, 1.0), vec3(0.0, 0.0, 0.0), check);

  gl_FragColor = vec4(c, 1.0);
}
