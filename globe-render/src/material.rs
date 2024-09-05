use bevy::{
    prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::Material2d
};

#[derive(Asset, TypePath, AsBindGroup, Clone)]
#[repr(C)]
pub struct RadialGradientMaterial {  
    #[uniform(0)]
    pub colors: [LinearRgba; 3],
    // #[storage(1)]
    // pub segments: Vec<f32>, 
    #[uniform(2)]
    pub center: Vec3,
}

impl Material2d for RadialGradientMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/radial_gradient.wgsl".into()
    }
}   
