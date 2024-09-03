#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> from_color: vec4<f32>;
@group(2) @binding(1) var<uniform> to_color: vec4<f32>;
@group(2) @binding(2) var<uniform> center: vec3<f32>;
@group(2) @binding(3) var<uniform> start_at: f32;
@group(2) @binding(4) var<uniform> end_at: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let fragment_radius = distance(mesh.world_position.xyz, center);
    if fragment_radius < start_at {
        return from_color;
    }

    let blend = (fragment_radius - start_at) / (end_at - start_at);
    return mix(from_color, to_color, vec4(blend, blend, blend, blend));
}