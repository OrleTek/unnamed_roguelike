#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;
layout(location = 1) out vec4 v_fgColor;
layout(location = 2) out vec4 v_bgColor;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

// TODO: merge dimensions into "sprites" buffer when that is supported in the Uniforms derive abstraction
layout(set = 1, binding = 0) uniform TextureAtlas_size {
    vec2 AtlasSize;
};

struct Rect {
    vec2 begin;
    vec2 end;
};

layout(set = 1, binding = 1) buffer TextureAtlas_textures {
    Rect[] Textures;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 SpriteTransform;
};

layout(set = 3, binding = 0) uniform ColorAtlas_size {
    vec2 ColorSize;
};

layout(set = 3, binding = 1) buffer ColorAtlas_colors {
    vec4[] Colors;
};

void main() {
    Rect sprite_rect = Textures[int(Vertex_Normal.x)];
    vec2 sprite_dimensions = vec2(16.0,16.0);
    vec3 vertex_position = Vertex_Position * vec3(sprite_dimensions, 1.0);

    vec2 uv = vec2(0,0);
    uv.x = mix( sprite_rect.begin.x, sprite_rect.end.x, Vertex_Uv.x);
    uv.y = mix( sprite_rect.begin.y, sprite_rect.end.y, Vertex_Uv.y);

    v_Uv = (uv + vec2(0.01, 0.01)) / AtlasSize;
    v_fgColor = Colors[int(Vertex_Normal.y)];
    v_bgColor = Colors[int(Vertex_Normal.z)];
    gl_Position = ViewProj * SpriteTransform * vec4(ceil(vertex_position), 1.0);
}