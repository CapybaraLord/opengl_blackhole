use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Scancode,
};

use crate::{objects::Uniform, winsdl::Winsdl};

pub mod objects;
mod winsdl;

fn main() {
    let mut winsdl = Winsdl::new(800, 800).unwrap();
    unsafe {
        gl::Viewport(0, 0, 800, 800);
    }

    // Shader/Program stuff
    let mut program = objects::create_program().unwrap();
    program.set();
    // Shader Uniform Locations
    let u_resolution = Uniform::new(program.id(), "u_resolution").unwrap();
    u_resolution.set_vec2f((800.0, 800.0));

    #[rustfmt::skip]
    let vertices = vec![
        -0.5, -0.5,
        0.5, -0.5,
        0.5, 0.5,
        -0.5, 0.5,
    ];

    let indices = vec![0, 3, 1, 1, 3, 2];

    let vbo = objects::Vbo::generate();
    vbo.set(&vertices);

    let vao = objects::Vao::generate();
    vao.set();

    let ibo = objects::Ibo::generate();
    ibo.set(&indices);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => unsafe {
                        gl::Viewport(0, 0, width, height);
                        u_resolution.set_vec2f((width as f32, height as f32));
                    },
                    _ => (),
                },
                Event::KeyDown { scancode, .. } => {
                    if let Some(scancode) = scancode {
                        match scancode {
                            Scancode::R => {
                                drop(program);
                                program = objects::create_program().unwrap();
                                program.set();
                            }
                            Scancode::Escape => break 'running,
                            _ => {}
                        }
                    }
                }
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Render Loop
        unsafe {
            gl::ClearColor(0.9, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        winsdl.window.gl_swap_window();
    }
}
