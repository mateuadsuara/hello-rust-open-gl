#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Surface, Program, VertexBuffer};
use glium::index::{PrimitiveType, NoIndices};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::glutin::{WindowBuilder, Event, ElementState, VirtualKeyCode};

fn main() {
    let display = create_display();

    let shape = vec![
        vec2(-0.5, -0.5),
        vec2( 0.0,  0.5),
        vec2( 0.5, -0.25)
    ];

    let mut offset: f32 = -0.5;
    loop {
        offset = update_offset(offset);
        draw(&display, &shape, offset);

        for ev in display.poll_events() {
            match ev {
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => ()
            }
        }
    }
}

fn update_offset(offset: f32) -> f32 {
    let mut t: f32 = offset;

    t += 0.0002;
    if t > 0.5 {
        t = -0.5;
    }

    return t;
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

fn draw(display: &GlutinFacade, shape: &Vec<Vertex>, offset: f32) {
    let vertex_shader_src = r#"
        #version 140

        uniform float offset;
        in vec2 position;

        void main() {
            gl_Position = vec4(position.x + offset, position.y, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 0.0);

    frame.draw(
        &VertexBuffer::new(display, shape).unwrap(),
        &NoIndices(PrimitiveType::TrianglesList),
        &Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        &uniform! {offset: offset},
        &Default::default()
    ).unwrap();

    frame.finish().unwrap();
}
