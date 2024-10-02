use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Default, Clone)]
pub struct OrbitTrailMaterial {
    #[uniform(0)]
    pub center: Vec3,
    #[uniform(1)]
    pub origin: Vec3,
    #[uniform(2)]
    pub background_color: Vec4,
    #[uniform(3)]
    pub trail_color: Vec4,
    #[uniform(4)]
    pub trail_theta: f32,
    #[uniform(5)]
    pub clockwise: u32,
}

impl Material2d for OrbitTrailMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/orbit_trail.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

impl Material for OrbitTrailMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/orbit_trail.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
