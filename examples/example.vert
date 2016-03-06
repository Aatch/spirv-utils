#version 430

layout(constant_id = 5) const bool enable_thing = false;

layout(location = 0) in vec2 i_position;
layout(location = 1) in vec3 i_color;

layout(set=0, binding=0) uniform anon { mat4 foo; };

out vec2 v_texcoords;
out vec4 v_color;

vec4 calc_pos(vec2 p) {
    return vec4(p, 0.0, 1.0);
}

void main() {
    v_texcoords = i_position;
    vec4 pos = calc_pos(i_position);

    v_color = vec4(i_color, 0.0);

    gl_Position = pos;
}

