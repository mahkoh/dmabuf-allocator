#version 450

layout(push_constant) uniform Push {
    ivec4 channels;
    vec2 size;
    vec2 subsampling;
    float angle;
} push;

layout(location = 0) out vec4 out_color;

const vec2 POSITIONS[3] = vec2[](vec2(0.0, -0.6), vec2(0.52, 0.3), vec2(-0.52, 0.3));
const vec3 COLORS[3] = vec3[](vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0));
const vec3 BACKGROUND = vec3(0.1);

float cross2(vec2 a, vec2 b) {
    return a.x * b.y - a.y * b.x;
}

vec3 color(vec2 p) {
    vec2 a = POSITIONS[0];
    vec2 b = POSITIONS[1];
    vec2 c = POSITIONS[2];
    float area = cross2(b - a, c - a);
    float w0 = cross2(b - p, c - p) / area;
    float w1 = cross2(c - p, a - p) / area;
    float w2 = 1.0 - w0 - w1;
    if (w0 < 0.0 || w1 < 0.0 || w2 < 0.0) {
        return BACKGROUND;
    }
    return w0 * COLORS[0] + w1 * COLORS[1] + w2 * COLORS[2];
}

vec3 yuv(vec3 rgb) {
    float y = dot(rgb, vec3(65.481, 128.553, 24.966)) + 16.0;
    float u = dot(rgb, vec3(-37.797, -74.203, 112.0)) + 128.0;
    float v = dot(rgb, vec3(112.0, -93.786, -18.214)) + 128.0;
    return vec3(y, u, v) / 255.0;
}

float select_channel(int channel, vec3 rgb, vec3 yuv) {
    if (channel < 3) {
        return rgb[channel];
    }
    if (channel < 6) {
        return yuv[channel - 3];
    }
    return 1.0;
}

void main() {
    vec2 pixel = gl_FragCoord.xy * push.subsampling;
    vec2 p = pixel / push.size * 2.0 - 1.0;
    p.x *= push.size.x / push.size.y;
    float c = cos(-push.angle);
    float s = sin(-push.angle);
    vec3 rgb = color(vec2(c * p.x - s * p.y, s * p.x + c * p.y));
    vec3 yuv = yuv(rgb);
    out_color = vec4(
        select_channel(push.channels.x, rgb, yuv),
        select_channel(push.channels.y, rgb, yuv),
        select_channel(push.channels.z, rgb, yuv),
        select_channel(push.channels.w, rgb, yuv)
    );
}
