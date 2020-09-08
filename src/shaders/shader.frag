#version 450

layout(set = 1, binding = 0) uniform sampler2D u_sampler;

layout(location = 0) in vec2 v_tc;
layout(location = 1) in vec4 v_color;

layout(location = 1) out vec4 o_color;

void main() {
  o_color = v_color;
  // o_color.a *= texture2D(u_sampler, v_tc).g;
}
