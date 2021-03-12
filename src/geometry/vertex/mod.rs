#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub position_: [f32; 3],
    pub normal_: [f32; 3],
    pub color_: [f32; 3],
    pub uv_: [f32; 2],
}
