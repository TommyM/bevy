use bevy_math::{Size, Vec2};
use bevy_render::renderer::RenderResources;

#[derive(Debug, Clone, Default, RenderResources)]
pub struct Node {
    pub size: Vec2,
}

#[derive(Default, Copy, Clone)]
pub struct CalculatedSize {
    pub size: Size,
}
