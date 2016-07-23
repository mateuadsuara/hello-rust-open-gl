#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Surface, Program, VertexBuffer, IndexBuffer};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::glutin::{WindowBuilder, Event, ElementState, VirtualKeyCode};

fn main() {
    let display = create_display();

    loop {
        draw(&display, &cube(), 0.0);

        for ev in display.poll_events() {
            match ev {
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => ()
            }
        }
    }
}

struct Shape {
    vertices: Vec<Vertex>,
    indices: Vec<u32>
}

fn plane() -> Shape {
    return Shape {
        vertices: vec![
            vec3(-1.0,-1.0, 0.0),
            vec3(-1.0, 1.0, 0.0),
            vec3( 1.0,-1.0, 0.0),
            vec3( 1.0, 1.0, 0.0),
        ],
        indices: vec![
            0, 1, 2,
            1, 2, 3
        ]
    };
}

fn cube() -> Shape {
    return Shape {
        vertices: vec![
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 1.0, 1.0),
            vec3(1.0, 0.0, 0.0),
            vec3(1.0, 0.0, 1.0),
            vec3(1.0, 1.0, 0.0),
            vec3(1.0, 1.0, 1.0),
        ],
        indices: vec![
            1,7,5,
            1,3,7, 
            1,4,3, 
            1,2,4, 
            3,8,7, 
            3,4,8, 
            5,7,8, 
            5,8,6, 
            1,5,6, 
            1,6,2, 
            2,6,8, 
            2,8,4, 
        ]
    };
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

fn vec3(x: f32, y: f32, z: f32) -> Vertex {
    return Vertex { position: [x, y, z] };
}

fn create_display() -> GlutinFacade {
    let monitor = glium::glutin::get_primary_monitor();
    return WindowBuilder::new()
        .with_fullscreen(monitor)
        .build_glium().unwrap();
}

fn draw(display: &GlutinFacade, shape: &Shape, offset: f32) {
    let vertex_shader_src = r#"
        #version 140

        uniform float offset;
        in vec3 position;

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
        &VertexBuffer::new(display, &shape.vertices).unwrap(),
        &IndexBuffer::new(display, PrimitiveType::TrianglesList, &shape.indices).unwrap(),
        &Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        &uniform! {offset: offset},
        &Default::default()
    ).unwrap();

    frame.finish().unwrap();
}
