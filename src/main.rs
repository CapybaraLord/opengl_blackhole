use sdl2::{event::Event, keyboard::Scancode};

use crate::winsdl::Winsdl;

pub mod objects;
mod winsdl;

fn main() {
    let mut winsdl = Winsdl::new(800, 600).unwrap();
    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let program = objects::create_program().unwrap();
    program.set();

    #[rustfmt::skip]
    let vertices = vec![
        -0.5, -0.5,
        0.5, -0.5,
        0.0, 0.5,
    ];

    let indices = vec![0, 1, 2];

    let vbo = objects::Vbo::generate();
    vbo.set(&vertices);

    let vao = objects::Vao::generate();
    vao.set();

    let ibo = objects::Ibo::generate();
    ibo.set(&indices);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break 'running,
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
