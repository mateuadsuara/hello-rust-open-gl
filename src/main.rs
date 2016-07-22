#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Surface, Program, VertexBuffer};
use glium::index::{PrimitiveType, NoIndices};
use glium::uniforms::{EmptyUniforms};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::glutin::{Event, WindowBuilder};

fn main() {
    let display = create_display();

    let shape = vec![
        vec2(-0.5, -0.5),
        vec2( 0.0,  0.5),
        vec2( 0.5, -0.25)
    ];

    loop {
        draw(&display, &shape);

        for ev in display.poll_events() {
            match ev {
                Event::Closed => {
                    println!("one");
                    return;
                },
                _ => ()
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn vec2(x: f32, y: f32) -> Vertex {
    return Vertex { position: [x, y] };
}

fn create_display() -> GlutinFacade {
    let monitor = glium::glutin::get_primary_monitor();
    return WindowBuilder::new()
        .with_fullscreen(monitor)
        .build_glium().unwrap();
}

fn draw(display: &GlutinFacade, shape: &Vec<Vertex>) {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let vertex_buffer = VertexBuffer::new(display, shape).unwrap();
    let indices = NoIndices(PrimitiveType::TrianglesList);
    let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.draw(&vertex_buffer, &indices, &program, &EmptyUniforms, &Default::default()).unwrap();
    frame.finish().unwrap();
}
