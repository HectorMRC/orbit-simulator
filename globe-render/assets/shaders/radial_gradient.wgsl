#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<storage, read> colors: array<vec4<f32>>;
@group(2) @binding(1) var<storage, read> segments: array<f32>;
@group(2) @binding(2) var<uniform> center: vec3<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> { 
    let fragment_radius = distance(mesh.world_position.xyz, center);

    var final_color = colors[0];
    for (var i = 0u; i < arrayLength(&segments); i++) {
        if fragment_radius < segments[i] {
            if i > 0 {
                let blend = (fragment_radius - segments[i-1]) / (segments[i] - segments[i-1]);
                final_color = mix(colors[i-1], colors[i], vec4(blend, blend, blend, blend));
            }

            break;
        }    

        if i == arrayLength(&segments) - 1 {
            final_color = colors[i];
        }
    }

    return final_color;
}