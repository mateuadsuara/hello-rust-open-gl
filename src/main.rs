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

mod counter;
use counter::ConsecutiveCounter;

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

fn obj_w_normals(vertices: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> Object {
    Object {
        vertices: vertices.chunks(3).map(|v| vec3(v[0], v[1], v[2])).collect(),
        normals: normals.chunks(3).map(|n| norm(n[0], n[1], n[2])).collect(),
        indices: indices,
    }
}

fn obj(vs: Vec<f32>, is: Vec<u32>) -> Object {
    obj_w_normals(vs, vec![], is)
}

fn plane() -> Object {
    obj(
        vec![
            -0.5, 0.0, 0.5,
             0.5, 0.0, 0.5,
             0.5, 0.0,-0.5,
            -0.5, 0.0,-0.5,
             0.0, 0.0, 0.0
        ],
        vec![
            0, 1, 4, 1, 2, 4, 2, 3, 4, 3, 0, 4
        ]
    )
}

fn sphere() -> Object {
    obj(
        vec![
        0.000000,0.850651,0.525731,
        -0.309017,0.500000,0.809017,
        0.309017,0.500000,0.809017,
        -0.525731,0.000000,0.850651,
        0.000000,0.000000,1.000000,
        0.525731,0.000000,0.850651,
        -0.850651,0.525731,0.000000,
        -0.809017,0.309017,0.500000,
        -0.500000,0.809017,0.309017,
        0.000000,0.850651,-0.525731,
        -0.500000,0.809017,-0.309017,
        0.000000,1.000000,0.000000,
        0.500000,0.809017,-0.309017,
        0.500000,0.809017,0.309017,
        0.850651,0.525731,0.000000,
        0.809017,0.309017,0.500000,
        0.850651,-0.525731,0.000000,
        1.000000,0.000000,0.000000,
        0.809017,-0.309017,0.500000,
        0.525731,0.000000,-0.850651,
        0.809017,0.309017,-0.500000,
        0.809017,-0.309017,-0.500000,
        0.309017,0.500000,-0.809017,
        -0.525731,0.000000,-0.850651,
        -0.309017,0.500000,-0.809017,
        0.000000,0.000000,-1.000000,
        0.000000,-0.850651,-0.525731,
        -0.309017,-0.500000,-0.809017,
        0.309017,-0.500000,-0.809017,
        0.500000,-0.809017,-0.309017,
        0.000000,-0.850651,0.525731,
        0.000000,-1.000000,0.000000,
        0.500000,-0.809017,0.309017,
        -0.850651,-0.525731,0.000000,
        -0.500000,-0.809017,-0.309017,
        -0.500000,-0.809017,0.309017,
        -0.809017,-0.309017,0.500000,
        -0.309017,-0.500000,0.809017,
        0.309017,-0.500000,0.809017,
        -1.000000,0.000000,0.000000,
        -0.809017,-0.309017,-0.500000,
        -0.809017,0.309017,-0.500000],

        vec![ 1, 0, 2,4, 3, 1,2, 5, 4,4, 1, 2,7, 6, 8,1, 3, 7,8, 0, 1,1, 7, 8,10, 9, 11,8, 6, 10,
        11, 0, 8,8, 10, 11,11, 9, 12,13, 0, 11,12, 14, 13,13, 11, 12,13, 14, 15,2, 0, 13,15, 5, 2,2, 13, 15,
        17, 16, 18,15, 14, 17,18, 5, 15,15, 17, 18,20, 19, 21,17, 14, 20,21, 16, 17,17, 20, 21,22, 19, 20,12, 9, 22,
        20, 14, 12,12, 22, 20,24, 23, 25,22, 9, 24,25, 19, 22,22, 24, 25,27, 26, 28,25, 23, 27,28, 19, 25,25, 27, 28,
        29, 16, 21,28, 26, 29,21, 19, 28,28, 29, 21,31, 30, 32,29, 26, 31,32, 16, 29,29, 31, 32,34, 33, 35,31, 26, 34,
        35, 30, 31,31, 34, 35,36, 3, 37,35, 33, 36,37, 30, 35,35, 36, 37,4, 5, 38,37, 3, 4,38, 30, 37,37, 4, 38,
        38, 5, 18,32, 30, 38,18, 16, 32,32, 38, 18,7, 3, 36,39, 6, 7,36, 33, 39,39, 7, 36,39, 33, 40,41, 6, 39,
        40, 23, 41,41, 39, 40,41, 23, 24,10, 6, 41,24, 9, 10,10, 41, 24,27, 23, 40,34, 26, 27,40, 33, 34,34, 27, 40]

    )
}

fn cube() -> Object {
    obj(
        vec![
        -0.5,-0.5, 0.5,
        0.5,-0.5, 0.5,
        0.5, 0.5, 0.5,
        -0.5, 0.5, 0.5,
        -0.5,-0.5, -0.5,
        0.5,-0.5, -0.5,
        0.5, 0.5, -0.5,
        -0.5, 0.5, -0.5
        ],
        vec![
        0, 1, 2, 0, 2, 3,
        1, 5, 6, 1, 6, 2,
        3, 2, 5, 3, 6, 7,
        4, 6, 5, 4, 7, 6,
        0, 7, 4, 0, 3, 7,
        0, 5, 1, 0, 4, 5
        ]
    )
}



fn main() {
    let display = create_display();
    let start_time = TimeMeasure::start();
    let object = cube();

    let mut view = View::new(
        [4., 0., -1.], [-2., 0., 1.], [0., 1., 0.]);

    let mut frames = ConsecutiveCounter::new();
    loop {
        let t = start_time.end();

        frames = frames.add_for(t.current_second(), 1);
        frames.completed_count().map(|(second, amount)|
            println!("{} frames on second {}.", amount, second));

        match poll_action(&display) {
            Action::Quit => return,
            Action::MoveView { delta: d } => view = view.add(&d),
        }

        let transformation = Mat4::new()
            .scale([(t.period(3.) * PI).sin() * 3., 1., 1.])
            .rotate((t.period(6.) * PI * 2.).sin(), [1., 0., 0.])
            .translate([0.5, 0., 0.])
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
    let perspective = Matrices::perspective(frame.get_dimensions());

    frame.clear_color(0., 0., 0., 0.);

    frame.draw(
        (
            &VertexBuffer::new(display, &object.vertices).unwrap(),
            &VertexBuffer::new(display, &object.normals).unwrap()
        ),
        &IndexBuffer::new(display, PrimitiveType::TrianglesList, &object.indices).unwrap(),
        &Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        &uniform! {
            view: Matrices::view(view),
            model: *transformation,
            perspective: perspective
        },
        &Default::default()
    ).unwrap();

    frame.finish().unwrap();
}
