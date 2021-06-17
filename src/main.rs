#![windows_subsystem = "windows"]

use failure;
use glium;

pub mod assets;
pub mod measurements;
pub mod misc;
pub mod platform;

use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::thread;
use std::time::Duration;

use failure::Error;
use glium::draw_parameters::{ DrawParameters, Blend };
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::{ PhysicalSize, PhysicalPosition };
use glium::glutin::event::{ Event, WindowEvent };
use glium::glutin::event_loop::{ ControlFlow, EventLoop };
use glium::glutin::window::WindowBuilder;
use glium::index::PrimitiveType;
use glium::{ Display, program, Surface };

use assets::Frames;
use measurements::Measurements;
use misc::{ fatal, UserEvent };

fn main() -> Result<(), Error> {
    // Create the event loop. This is what takes in events like keypresses and
    // clicks from the operating system.
    let event_loop = EventLoop::<UserEvent>::with_user_event();
    
    // Take some measurements of the screen(s), so we know how to correctly
    // scale the window. This gets passed the event loop because the event loop
    // has a function that lets you check the size and position of connected
    // monitors.
    let mut measurements = Measurements::new(&event_loop);

    // Create a "display". A display is an abstraction provided by the glium
    // crate that lets us easily render to an OpenGL canvas without all the
    // headaches associated with it.
    let display = {
        let window_builder = WindowBuilder::new()
            .with_title("a")
            .with_resizable(false)
            .with_transparent(true)
            .with_always_on_top(true)
            .with_decorations(false)
            .with_inner_size(PhysicalSize {
                width: measurements.shark_size.0,
                height: measurements.shark_size.1,
            });

        let context_builder = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);

        Display::new(
            window_builder,
            context_builder,
            &event_loop)?
    };

    // Move the window to where it should be.
    display.gl_window().window().set_outer_position(PhysicalPosition {
        x: measurements.shark_pos.0,
        y: measurements.shark_pos.1,
    });
    
    // If you're on windows, theres a few more steps to do. Namely, hide the
    // icon for the window in the taskbar and to make it so you can click the 
    // windows behind this window when you click it. Additionally, we set up a 
    // system tray menu that lets the user close the app easily.
    #[cfg(platform_windows)] { 
        platform::windows::configure_window(&display);
        platform::windows::configure_tray();
    }

    let vertex_buffer = {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        glium::implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex { position: [ -1.0, -1.0 ], tex_coords: [ 0.0, 0.0 ] },
                Vertex { position: [ -1.0,  1.0 ], tex_coords: [ 0.0, 1.0 ] },
                Vertex { position: [  1.0,  1.0 ], tex_coords: [ 1.0, 1.0 ] },
                Vertex { position: [  1.0, -1.0 ], tex_coords: [ 1.0, 0.0 ] },
            ])?
    };

    let index_buffer = glium::IndexBuffer::new(
        &display,
        PrimitiveType::TriangleStrip,
        &[1 as u16, 2, 0, 3])?;

    let program = glium::program!(
        &display,
        140 => {
            vertex: include_str!("vertex.140.glsl"),
            fragment: include_str!("fragment.140.glsl"),
        },
        110 => {
            vertex: include_str!("vertex.110.glsl"),
            fragment: include_str!("fragment.110.glsl"),
        },
        100 => {
            vertex: include_str!("vertex.100.glsl"),
            fragment: include_str!("fragment.100.glsl"),
        })?;
    
    // This stores whether or not the frames are done loading.
    //
    // Arc<AtomicBool> allows us to create multiple references to the same
    // value that all update when one of them is changed.
    let ready = Arc::new(AtomicBool::new(false));

    // Create a handle to the loaded frames. Note that they're not actually
    // loaded yet, but this Frames type blocks until the next frame has fully
    // loaded when a frame is requested.
    // Display is passed so that it can resize the images to fit the window.
    let mut frames = Frames::new(&display, ready.clone());
 
    let mut go_right = true;

    // On Windows, transparent windows are repainted whenever the content
    // below them changes. So, when the window is moved, the content below it
    // is different. Since we move the window at twice the speed of the frames,
    // we need to keep track of the current frame number, so that we can check
    // if this is an even frame when repainting the window.
    #[allow(unused_mut)] let mut frame_count: usize = 0;
    
    // Start up a thread that moves the window periodically.
    {
        let event_loop_proxy = event_loop.create_proxy();
        let ready = ready.clone();
        thread::spawn(move || {
            // Wait for the assets to be ready.
            loop {
                if ready.load(Ordering::Relaxed) { break; }
                thread::sleep(Duration::from_millis(100));
            }
            // Send an event to move the window every 50ms.
            loop {
                match event_loop_proxy.send_event(UserEvent::Move) {
                    Ok(()) => (),
                    Err(error) => fatal("An unexpected runtime error occured.", format!("{}", error)),
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    // See the comment above where `frame_count` is defined.
    #[cfg(not(platform_windows))] {
        let event_loop_proxy = event_loop.create_proxy();
        let ready = ready.clone();
        thread::spawn(move || {
            // Wait for the assets to be ready.
            loop {
                if ready.load(Ordering::Relaxed) { break; }
                thread::sleep(Duration::from_millis(100));
            }
            // Send an event to advance the frame every 100ms.
            loop {
                match event_loop_proxy.send_event(UserEvent::Frame) {
                    Ok(()) => (),
                    Err(error) => fatal("An unexpected runtime error occured.", format!("{}", error)),
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(UserEvent::Move) => {
                if go_right {
                    measurements.shark_pos.0 += 3;
                    display.gl_window().window().set_outer_position(PhysicalPosition {
                        x: measurements.shark_pos.0,
                        y: measurements.shark_pos.1,
                    });

                    let bound = measurements.area_max_pos.0;
                    
                    if measurements.shark_pos.0 > bound { go_right = false; }
                } else {
                    measurements.shark_pos.0 -= 3; 
                    display.gl_window().window().set_outer_position(PhysicalPosition {
                        x: measurements.shark_pos.0,
                        y: measurements.shark_pos.1,
                    });

                    let bound = measurements.area_min_pos.0 - measurements.shark_size.0;
                    
                    if measurements.shark_pos.0 < bound { go_right = true; }
                }
            },
            Event::UserEvent(UserEvent::Frame) => {
                display.gl_window().window().request_redraw();
            },
            Event::RedrawRequested(..) => {
                // See the comment above where `frame_count` is defined.
                if cfg!(not(platform_windows)) || frame_count % 2 == 0 {
                    let frame = frames.next().unwrap();
                    let uniform = glium::uniform! {
                        matrix: [
                            [ 1.0, 0.0, 0.0, 0.0 ],
                            [ 0.0, 1.0, 0.0, 0.0 ],
                            [ 0.0, 0.0, 1.0, 0.0 ],
                            [ 0.0, 0.0, 0.0, 1.0f32 ],
                        ],
                        tex: &*frame,
                    };
                    let mut display = display.draw();
                    display.clear_color(0.0, 0.0, 0.0, 0.0);
                    let _ = display.draw(
                        &vertex_buffer,
                        &index_buffer,
                        &program,
                        &uniform,
                        &DrawParameters {
                            blend: Blend::alpha_blending(),
                            ..Default::default()
                        });
                    let _ = display.finish();
                }
                #[cfg(platform_windows)] {
                    frame_count += 1;
                    if frame_count == usize::MAX { frame_count = 0; }
                }
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => (),
        }
    });

}

