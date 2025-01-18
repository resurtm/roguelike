use crate::{
    consts::{WINDOW_SIZE, WINDOW_TITLE},
    video::VideoState,
};
use std::sync::Arc;
use thiserror::Error;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    error::{EventLoopError, OsError},
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub async fn run() -> Result<(), RunError> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(WINDOW_SIZE.0, WINDOW_SIZE.1))
            .with_position(PhysicalPosition::new(50, 50))
            .with_title(WINDOW_TITLE)
            .build(&event_loop)?,
    );
    let mut video_state = VideoState::new(Arc::clone(&window)).await;
    let mut surface_ready = false;

    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent { ref event, window_id } if window_id == window.id() => match event {
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
                video_state.resize(*physical_size);
                surface_ready = true;
            }

            WindowEvent::RedrawRequested => {
                window.request_redraw();
                if !surface_ready {
                    return;
                }
                match video_state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        video_state.resize(video_state.size);
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
        },

        _ => {}
    })?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error("event loop error: {0}")]
    EventLoopError(#[from] EventLoopError),

    #[error("os error: {0}")]
    OsError(#[from] OsError),
}
