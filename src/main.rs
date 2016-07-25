#[macro_use]

extern crate glium;
use glium::{DisplayBuild, Surface, Program, VertexBuffer, IndexBuffer};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::glutin::{WindowBuilder, Event};
use glium::glutin::ElementState::*;
use glium::glutin::VirtualKeyCode::*;

use std::f32::consts::{PI};

mod time_measure;
use time_measure::*;

mod matrices;
use matrices::*;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

fn vec3(x: f32, y: f32, z: f32) -> Vertex {
    Vertex { position: [x, y, z] }
}

#[derive(Copy, Clone)]
pub struct Normal {
    normal: [f32; 3],
}
implement_vertex!(Normal, normal);

fn norm(x: f32, y: f32, z: f32) -> Normal {
    Normal { normal: [x, y, z] }
}

struct Object {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    normals: Vec<Normal>
}

fn cube() -> Object {
    Object {
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
        normals: vec! [
            norm( 0.0, 0.0,-1.0),
            norm( 0.0, 0.0,-1.0),
            norm(-1.0, 0.0, 0.0),
            norm(-1.0, 0.0, 0.0),
            norm( 0.0, 1.0, 0.0),
            norm( 0.0, 1.0, 0.0),
            norm( 1.0, 0.0, 0.0),
            norm( 1.0, 0.0, 0.0),
            norm( 0.0,-1.0, 0.0),
            norm( 0.0,-1.0, 0.0),
            norm( 0.0, 0.0, 1.0),
            norm( 0.0, 0.0, 1.0),
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
        ],
    }
}



fn main() {
    let display = create_display();
    let start_time = TimeMeasure::start();
    let object = cube();

    let mut view = View::new(
        [4., 0., -1.], [-2., 0., 1.], [0., 1., 0.]);

    loop {
        match poll_action(&display) {
            Action::Quit => return,
            Action::MoveView { delta: d } => view = view.add(&d),
        }

        let t = start_time.end();
        let transformation = Mat4::new()
            .scale([(t.period(3.) * PI).sin() * 3., 1., 1.])
            .rotate((t.period(6.) * PI * 2.).sin(), [1., 0., 0.])
            .translate([0., -0.5, -0.5])
            .end();
        draw(&display, (&object, &transformation), &view);
    }
}


enum Action {
    Quit,
    MoveView { delta: View }
}

fn poll_action(display: &GlutinFacade) -> Action {
    let mut delta_view = View::neutral();

    for ev in display.poll_events() {
        match ev {
            Event::KeyboardInput(Pressed, _, Some(Escape)) => return Action::Quit,
            Event::KeyboardInput(Pressed, _, Some(D)) => delta_view.position[2] += 1.,
            Event::KeyboardInput(Pressed, _, Some(A)) => delta_view.position[2] -= 1.,
            Event::KeyboardInput(Pressed, _, Some(W)) => delta_view.position[1] += 1.,
            Event::KeyboardInput(Pressed, _, Some(S)) => delta_view.position[1] -= 1.,
            Event::KeyboardInput(Pressed, _, Some(E)) => delta_view.position[0] -= 1.,
            Event::KeyboardInput(Pressed, _, Some(Q)) => delta_view.position[0] += 1.,
            _ => ()
        }
    }

    Action::MoveView { delta: delta_view }
}

fn create_display() -> GlutinFacade {
    //let monitor = glium::glutin::get_primary_monitor();
    return WindowBuilder::new()
        //.with_fullscreen(monitor)
        .build_glium().unwrap();
}

fn draw(display: &GlutinFacade, model: (&Object, &Matrix), view: &View) {
    let (object, transformation) = model;
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 view;
        uniform mat4 model;
        uniform mat4 perspective;

        void main() {
            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let mut frame = display.draw();
    let dimensions = frame.get_dimensions();
    let indices = object.indices.iter().map(|&i| i - 1).collect::<Vec<_>>();

    frame.clear_color(0., 0., 0., 0.);

    frame.draw(
        (
            &VertexBuffer::new(display, &object.vertices).unwrap(),
            &VertexBuffer::new(display, &object.normals).unwrap()
        ),
        &IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap(),
        &Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        &uniform! {
            view: Matrices::view(view),
            model: *transformation,
            perspective: Matrices::perspective(dimensions)
        },
        &Default::default()
    ).unwrap();

    frame.finish().unwrap();
}
