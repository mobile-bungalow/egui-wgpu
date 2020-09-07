#version 450

layout(set = 1, binding = 0) uniform sampler2D u_sampler;

layout(location = 0) in vec2 v_tc;
layout(location = 1) in vec4 v_color;

void main() {
  gl_FragColor = v_color;
  gl_FragColor.a *= texture2D(u_sampler, v_tc).g;
}
