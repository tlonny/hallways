struct CameraUniform {
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
    clip_plane: vec4<f32>,
}

struct MaterialEntry {
    num_frames: u32,
    speed: f32,
    offset: u32,
    color: u32,
    texture_addressing: u32,
}

struct MaterialIndex {
    entries: array<MaterialEntry, 512>,
    frames: array<u32, 4096>,
}

struct ClockUniform {
    clock: u32,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) diffuse_uv: vec2<f32>,
    @location(2) material_ix: u32,
    @location(3) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) diffuse_uv: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) @interpolate(flat) material_ix: u32,
    @location(3) color: vec4<f32>,
}

struct TransparentOutput {
    @location(0) accum: vec4<f32>,
    @location(1) reveal: f32,
}

const FRAME_DURATION_MS: u32 = 10u;
const ANIMATION_TICK_WRAP_PER_FRAME: u32 = 100u;
const TEXTURE_ADDRESSING_NEAREST: u32 = 1u;

// Group 0: Textures
@group(0) @binding(0)
var diffuse_sampler_linear: sampler;
@group(0) @binding(1)
var diffuse_sampler_nearest: sampler;
@group(0) @binding(2)
var diffuse_0: texture_2d_array<f32>;
@group(0) @binding(3)
var diffuse_1: texture_2d_array<f32>;
@group(0) @binding(4)
var diffuse_2: texture_2d_array<f32>;
@group(0) @binding(5)
var diffuse_3: texture_2d_array<f32>;
@group(0) @binding(6)
var diffuse_4: texture_2d_array<f32>;
@group(0) @binding(7)
var diffuse_5: texture_2d_array<f32>;

@group(0) @binding(8)
var<storage, read> material_index: MaterialIndex;

// Group 1: Config
@group(1) @binding(0)
var<uniform> camera: CameraUniform;
@group(1) @binding(1)
var<uniform> clock: ClockUniform;

fn unpack_bucket(texture_ref: u32) -> u32 {
    return texture_ref & 0xFFFFu;
}

fn unpack_layer(texture_ref: u32) -> u32 {
    return (texture_ref >> 16u) & 0xFFFFu;
}

fn unpack_color(color: u32) -> vec4<f32> {
    let r = f32(color & 0xFFu) / 255.0;
    let g = f32((color >> 8u) & 0xFFu) / 255.0;
    let b = f32((color >> 16u) & 0xFFu) / 255.0;
    let a = f32((color >> 24u) & 0xFFu) / 255.0;
    return vec4<f32>(r, g, b, a);
}

fn sample_diffuse(
    array_ix: u32,
    texture_sampler: sampler,
    uv: vec2<f32>,
    layer_ix: u32,
) -> vec4<f32> {
    if (array_ix == 0u) {
        return textureSample(diffuse_0, texture_sampler, uv, layer_ix);
    }
    if (array_ix == 1u) {
        return textureSample(diffuse_1, texture_sampler, uv, layer_ix);
    }
    if (array_ix == 2u) {
        return textureSample(diffuse_2, texture_sampler, uv, layer_ix);
    }
    if (array_ix == 3u) {
        return textureSample(diffuse_3, texture_sampler, uv, layer_ix);
    }
    if (array_ix == 4u) {
        return textureSample(diffuse_4, texture_sampler, uv, layer_ix);
    }
    return textureSample(diffuse_5, texture_sampler, uv, layer_ix);
}

fn sample_material(material_id: u32, uv: vec2<f32>) -> vec4<f32> {
    let mat = material_index.entries[material_id];

    if (mat.num_frames == 0u) {
        return unpack_color(mat.color);
    }

    let animation_tick_wrap = ANIMATION_TICK_WRAP_PER_FRAME * mat.num_frames;
    let animation_tick = (clock.clock / FRAME_DURATION_MS) % animation_tick_wrap;
    let animation_speed = clamp(mat.speed, 0.0, 1.0);
    let frame_offset =
        u32(floor(f32(animation_tick) * animation_speed)) % mat.num_frames;
    let texture_ref = material_index.frames[mat.offset + frame_offset];

    let array_ix = unpack_bucket(texture_ref);
    let layer_ix = unpack_layer(texture_ref);

    if (mat.texture_addressing == TEXTURE_ADDRESSING_NEAREST) {
        return sample_diffuse(array_ix, diffuse_sampler_nearest, uv, layer_ix)
            * unpack_color(mat.color);
    }

    let sampled = sample_diffuse(array_ix, diffuse_sampler_linear, uv, layer_ix);
    return sampled * unpack_color(mat.color);
}

fn fragment_clipped(world_position: vec3<f32>) -> bool {
    if (length(camera.clip_plane.xyz) == 0.0) {
        return false;
    }

    let dist = dot(world_position, camera.clip_plane.xyz) + camera.clip_plane.w;
    return dist < 0.0;
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = vec4<f32>(in.position, 1.0);
    let view_position = camera.view * world_position;

    out.clip_position = camera.projection * view_position;
    out.diffuse_uv = in.diffuse_uv;
    out.world_position = world_position.xyz;
    out.material_ix = in.material_ix;
    out.color = in.color;
    return out;
}

@fragment
fn fs_opaque(in: VertexOutput) -> @location(0) vec4<f32> {
    if (fragment_clipped(in.world_position)) {
        discard;
    }

    let diffuse_color = sample_material(in.material_ix, in.diffuse_uv) * in.color;
    if (diffuse_color.a < 1.0) {
        discard;
    }

    return diffuse_color;
}

@fragment
fn fs_transparent(in: VertexOutput) -> TransparentOutput {
    if (fragment_clipped(in.world_position)) {
        discard;
    }

    let diffuse_color = sample_material(in.material_ix, in.diffuse_uv) * in.color;
    if (diffuse_color.a <= 0.0 || diffuse_color.a >= 1.0) {
        discard;
    }

    let alpha = diffuse_color.a;
    return TransparentOutput(
        vec4<f32>(diffuse_color.rgb * alpha, alpha),
        alpha,
    );
}
