use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Default, Clone)]
pub struct RadialGradientMaterial {
    #[uniform(0)]
    pub from_color: LinearRgba,
    #[uniform(1)]
    pub to_color: LinearRgba,
    #[uniform(2)]
    pub center: Vec3,
    #[uniform(3)]
    pub start_at: f32,
    #[uniform(4)]
    pub end_at: f32,
}

impl Material2d for RadialGradientMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/radial_gradient.wgsl".into()
    }
}
