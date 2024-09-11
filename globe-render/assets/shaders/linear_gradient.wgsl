#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<storage, read> colors: array<vec4<f32>>;
@group(2) @binding(1) var<storage, read> segments: array<f32>;
@group(2) @binding(2) var<uniform> center: vec3<f32>;
@group(2) @binding(3) var<uniform> theta: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let translation_matrix = mat4x4(
        vec4(1., 0., 0., -center.x),
        vec4(0., 1., 0., -center.y),    
        vec4(0., 0., 1., -center.z),
        vec4(0., 0., 0., 1.),
    );

    let sin_theta = sin(theta);
    let cos_theta = cos(theta);
    let sub_1_cos_theta = 1. - cos_theta;

    let x = 0.;
    let y = 0.;
    let z = 1.;

    let rotation_matrix = mat4x4(
        vec4(
            cos_theta + pow(x, 2.) * sub_1_cos_theta,
            x * y * sub_1_cos_theta - z * sin_theta,
            x * z * sub_1_cos_theta + y * sin_theta,
            0.
        ),
        vec4(
            y * x * sub_1_cos_theta + z * sin_theta,
            cos_theta + pow(y, 2.) * sub_1_cos_theta,
            y * z * sub_1_cos_theta - x * sin_theta,
            0.
        ),    
        vec4(
            z * x * sub_1_cos_theta - y * sin_theta,
            z * y * sub_1_cos_theta + x * sin_theta,
            cos_theta + pow(z, 2.) * sub_1_cos_theta,
            0.
        ),
        vec4(0., 0., 0., 1.),
    );

    var fragment = mesh.world_position * translation_matrix;
    fragment = fragment * rotation_matrix;

    let fragment_y = fragment.y;

    var final_color = colors[0];
    for (var i = 0u; i < arrayLength(&segments); i++) {
        if fragment_y < segments[i] {
            if i > 0 {
                let blend = abs(fragment_y - segments[i-1]) / abs(segments[i] - segments[i-1]);
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