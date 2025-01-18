use crate::{
    consts::{WINDOW_SIZE, WINDOW_TITLE},
    video::VideoState,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .with_position(PhysicalPosition::new(50, 50))
        .with_title(WINDOW_TITLE)
        .build(&event_loop)
        .unwrap();

    let mut state = VideoState::new(&window).await;
    let mut surface_configured = false;

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent { ref event, window_id } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                            surface_configured = true;
                        }
                        WindowEvent::RedrawRequested => {
                            state.window().request_redraw();

                            if !surface_configured {
                                return;
                            }

                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    state.resize(state.size)
                                }
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    log::error!("OutOfMemory");
                                    control_flow.exit();
                                }
                                Err(wgpu::SurfaceError::Timeout) => {
                                    log::warn!("Surface timeout")
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        })
        .unwrap();
}
