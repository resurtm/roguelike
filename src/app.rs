use crate::video::VideoError;
use std::sync::Arc;
use thiserror::Error;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    error::{EventLoopError, OsError},
    event,
    event::WindowEvent,
    keyboard::{KeyCode, PhysicalKey},
};

/// Game application main entry point.
pub async fn launch() -> Result<(), LaunchError> {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1920, 1280))
            .with_position(PhysicalPosition::new(25, 25))
            .with_title("roguelike 🔮🧝🏻🪄")
            .build(&event_loop)?,
    );

    let mut video = crate::video::Video::new(Arc::clone(&window)).await?;
    let mut surface_ready = false;

    let mut input = crate::input::Input::new();
    let mut scene = crate::scene::Scene::new(&video)?;
    scene.observer.handle_resize(&video, window.inner_size().into());

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

                WindowEvent::KeyboardInput { event, .. } => input.handle_key_event(event),

                WindowEvent::Resized(physical_size) => {
                    video.handle_resize(*physical_size);
                    scene.observer.handle_resize(&video, (*physical_size).into());
                    surface_ready = true;
                }

                WindowEvent::RedrawRequested => {
                    // artificially slow down / cap frame rate
                    std::thread::sleep(std::time::Duration::new(0, FRAME_DELAY_NSECS));
                    scene.advance(&input);
                    window.request_redraw();
                    if !surface_ready {
                        return;
                    }
                    match video.render(&scene) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            video.handle_resize(window.inner_size());
                            scene.observer.handle_resize(&video, window.inner_size().into());
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

const FRAME_DELAY_NSECS: u32 = 1_000_000_000 / 60; // 1_000 msecs / 60

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
