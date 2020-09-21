#version 450

  const mat4 WGPU_CORRECTION = mat4(
      vec4(1., 0., 0., 0.),
      vec4(0., 1., 0., 0.),
      vec4(0., 0., 0.5, 0.5),
      vec4(0., 0., 0., 1.)
      );

  layout (set = 0, binding = 0) uniform Globals {
    vec2 u_screen_size;
    vec2 u_tex_size;
  };

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_tc;
layout(location = 2) in uvec4 a_color;

out gl_PerVertex {
  vec4 gl_Position;
};

layout(location = 0) out vec2 v_tc;
layout(location = 1) out vec4 v_color;

void main() {

  vec4 pos = vec4(a_pos, 0., 0.) * WGPU_CORRECTION;

  gl_Position = vec4(
      2.0 * pos.x / u_screen_size.x - 1.0,
      1.0 - 2.0 * pos.y / u_screen_size.y,
      0.0,
      1.0);

  v_tc = a_tc / u_tex_size;
  v_color = a_color / 255.0;
}

