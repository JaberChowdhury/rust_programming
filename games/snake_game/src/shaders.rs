pub const VERTEX_SHADER: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    color = color0;
    uv = texcoord;
}
"#;

pub const FRAGMENT_SHADER_ZEBRA: &str = r#"#version 100
precision lowp float;
varying vec2 uv;
varying vec4 color;
uniform vec3 base_color;
uniform float time;

void main() {
    float strip = step(0.5, fract(uv.x * 10.0 + uv.y * 10.0 + time * 2.0));
    gl_FragColor = vec4(mix(base_color, vec3(1.0), strip * 0.4), color.a);
}
"#;

pub const FRAGMENT_SHADER_PLASMA: &str = r#"#version 100
precision lowp float;
varying vec2 uv;
varying vec4 color;
uniform vec3 base_color;
uniform float time;

void main() {
    float v1 = sin(uv.x * 10.0 + time);
    float v2 = sin(10.0 * (uv.x * sin(time / 2.0) + uv.y * cos(time / 3.0)) + time);
    float cx = uv.x + 0.5 * sin(time / 5.0);
    float cy = uv.y + 0.5 * cos(time / 3.0);
    float v3 = sin(sqrt(100.0 * (cx * cx + cy * cy) + 1.0) + time);
    float vf = v1 + v2 + v3;
    float r = sin(vf * 3.14159) * 0.5 + 0.5;
    float g = sin(vf * 3.14159 + 2.0) * 0.5 + 0.5;
    float b = sin(vf * 3.14159 + 4.0) * 0.5 + 0.5;
    // Mix the base color with the plasma pattern (0.4 opacity) so the base color is preserved
    vec3 plasma = vec3(r, g, b);
    gl_FragColor = vec4(mix(base_color, plasma, 0.4), color.a);
}
"#;
