#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> center: vec3<f32>;
@group(2) @binding(1) var<uniform> origin: vec3<f32>;
@group(2) @binding(2) var<uniform> background_color: vec4<f32>;
@group(2) @binding(3) var<uniform> trail_color: vec4<f32>;
@group(2) @binding(4) var<uniform> trail_theta: f32;
@group(2) @binding(5) var<uniform> clockwise: u32;  

const PI = 3.14159265359;

// Given a radian in the range of [-π, π], returns the value in the range of [0, 2π].
fn positive_radiants(value: f32) -> f32 {
    if value < 0. {
        return value + 2*PI;
    } else {
        return value;
    }
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let translation_matrix = mat4x4(
        vec4(1., 0., 0., -center.x),
        vec4(0., 1., 0., -center.y),    
        vec4(0., 0., 1., -center.z),
        vec4(0., 0., 0., 1.),
    );

    var orientation = 1.; // counter-clockwise
    if bool(clockwise) {
        orientation = -1.;
    }

    let rel_origin = vec4(origin.x, origin.y, origin.z, 1.) * translation_matrix;
    let origin_theta = orientation * positive_radiants(atan2(rel_origin.y, rel_origin.x));

    let rel_world_position = mesh.world_position * translation_matrix;
    let fragment_theta = orientation * positive_radiants(atan2(rel_world_position.y, rel_world_position.x));

    var theta_diff = origin_theta - fragment_theta; 
    if origin_theta < fragment_theta {
        theta_diff = 2*PI - fragment_theta + origin_theta;
    }

    if theta_diff > trail_theta {
        return background_color;
    }
  
    let blend = theta_diff / trail_theta;
    return mix(trail_color, background_color, vec4(blend, blend, blend, blend));
}