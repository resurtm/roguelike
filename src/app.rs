use crate::{
    consts::{WINDOW_SIZE, WINDOW_TITLE},
    video::VideoError,
};
use std::sync::Arc;
use thiserror::Error;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    error::{EventLoopError, OsError},
    event,
    event::WindowEvent,
    keyboard::{KeyCode, PhysicalKey},
};

// Game application main entry point.
pub async fn launch() -> Result<(), LaunchError> {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new()?;
    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(WINDOW_SIZE.0, WINDOW_SIZE.1))
            .with_position(PhysicalPosition::new(50, 50))
            .with_title(WINDOW_TITLE)
            .build(&event_loop)?,
    );

    let mut video = crate::video::Video::new(Arc::clone(&window)).await?;
    let mut surface_ready = false;

    let scene = crate::scene::Scene::new(&video)?;

    event_loop.run(move |event, control_flow| match event {
        event::Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        event::KeyEvent {
                            state: event::ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),

                WindowEvent::Resized(physical_size) => {
                    video.handle_resize(*physical_size);
                    surface_ready = true;
                }

                WindowEvent::RedrawRequested => {
                    window.request_redraw();
                    if !surface_ready {
                        return;
                    }
                    match video.render(&scene) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            video.handle_resize(window.inner_size());
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

        _ => {}
    })?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum LaunchError {
    #[error("event loop error: {0}")]
    EventLoop(#[from] EventLoopError),

    #[error("os error: {0}")]
    Os(#[from] OsError),

    #[error("video error: {0}")]
    Video(#[from] VideoError),

    #[error("scene error: {0}")]
    Scene(#[from] crate::scene::SceneError),
}
