use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef},
        storage::ShaderStorageBuffer,
    },
    sprite::{AlphaMode2d, Material2d},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Default, Clone)]
pub struct RadialGradientMaterial {
    #[storage(0, read_only)]
    colors: Handle<ShaderStorageBuffer>,
    #[storage(1, read_only)]
    segments: Handle<ShaderStorageBuffer>,
    #[uniform(2)]
    center: Vec3,
}

impl Material2d for RadialGradientMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/radial_gradient.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

struct ColorSegment {
    color: [f32; 4],
    start: f32,
}

pub struct RadialGradientMaterialBuilder<'a> {
    buffer: &'a mut Assets<ShaderStorageBuffer>,
    segments: Vec<ColorSegment>,
    center: Vec3,
}

impl<'a> RadialGradientMaterialBuilder<'a> {
    pub fn new(buffer: &'a mut Assets<ShaderStorageBuffer>) -> Self {
        Self {
            buffer,
            segments: Default::default(),
            center: Default::default(),
        }
    }

    pub fn with_center(mut self, center: Vec3) -> Self {
        self.center = center;
        self
    }

    pub fn with_segment(mut self, color: Color, start: f32) -> Self {
        self.segments.push(ColorSegment {
            color: color.to_linear().to_f32_array(),
            start,
        });

        self
    }

    pub fn build(mut self) -> RadialGradientMaterial {
        self.segments.sort_by(|a, b| a.start.total_cmp(&b.start));

        let mut segments = Vec::with_capacity(self.segments.len());
        let mut colors = Vec::with_capacity(self.segments.len());
        self.segments.into_iter().for_each(|segment| {
            segments.push(segment.start);
            colors.push(segment.color);
        });

        RadialGradientMaterial {
            colors: self.buffer.add(ShaderStorageBuffer::new(
                bytemuck::cast_slice(colors.as_slice()),
                RenderAssetUsages::default(),
            )),
            segments: self.buffer.add(ShaderStorageBuffer::new(
                bytemuck::cast_slice(segments.as_slice()),
                RenderAssetUsages::default(),
            )),
            center: self.center,
        }
    }
}
