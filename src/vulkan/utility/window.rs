use winit::event_loop::EventLoop;

#[allow(dead_code)]
pub fn create_window(
    event_loop: &EventLoop<()>,
    header: &str,
    width: u32,
    height: u32,
) -> winit::window::Window {
    winit::window::WindowBuilder::new()
        .with_title(header)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(event_loop)
        .expect("Could not create window")
}
