#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(0) @binding(0) var<uniform> colors: array<vec4<f32>, 15>;
// @group(0) @binding(1) var<storage, read> segments: array<f32>;
@group(0) @binding(2) var<uniform> center: vec3<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {             
    return vec4(1., 0., 0., 1.);
}               