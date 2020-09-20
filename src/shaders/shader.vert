#version 450
//
//layout (set = 0, binding = 0) uniform Globals {
//  vec2 u_screen_size;
//  vec2 u_tex_size;
//};
//
//layout(location = 0) in vec2 a_pos;
//layout(location = 1) in vec2 a_tc;
//layout(location = 2) in vec4 a_color;
//
//
//void main() {

//  gl_Position = vec4(
//      2.0 * a_pos.x / u_screen_size.x - 1.0,
//      1.0 - 2.0 * a_pos.y / u_screen_size.y,
//      0.0,
//      1.0);
//
//  v_tc = a_tc / u_tex_size;
//  v_color = a_color / 255.0;
//}

out gl_PerVertex {
  vec4 gl_Position;
};

layout(location = 0) out vec2 v_tc;
layout(location = 1) out vec4 v_color;

void main(){
  vec2 position = vec2(gl_VertexIndex, (gl_VertexIndex & 1) * 2) - 1;
  gl_Position = vec4(position, 0.0, 1.0);
  v_tc = position.xy;
  v_color = vec4(position.xy, position.x / position.y, 1.);
  //v_color  = vec4(v_tc.xy, 1., 1.);
}
