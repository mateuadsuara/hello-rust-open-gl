#[macro_use]

extern crate glium;
use glium::{DisplayBuild, Surface, Program, VertexBuffer, IndexBuffer};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::glutin::{WindowBuilder};
use glium::glutin::Event::*;
use glium::glutin::ElementState::*;
use glium::glutin::VirtualKeyCode::*;


mod time_measure;
use time_measure::*;

mod matrices;
use matrices::*;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

struct Shape {
    vertices: Vec<Vertex>,
    indices: Vec<u32>
}

fn vec3(x: f32, y: f32, z: f32) -> Vertex {
    return Vertex { position: [x, y, z] };
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



fn main() {
    let display = create_display();
    let start_time = TimeMeasure::start();

    let mut view = View::new([6.0, 0.0, 0.0], [-2.0, 0.0, 1.0], [0.0, 1.0, 0.0]);
    let mut previous_mouse = None;

    loop {
        draw(&display, &cube(), &start_time.end(), &view);

        let mut delta_view = View::neutral();
        for ev in display.poll_events() {
            match ev {
                KeyboardInput(Pressed, _, Some(Escape)) => return,
                KeyboardInput(Pressed, _, Some(D)) => delta_view.position[2] += 1.0,
                KeyboardInput(Pressed, _, Some(A)) => delta_view.position[2] -= 1.0,
                KeyboardInput(Pressed, _, Some(W)) => delta_view.position[1] += 1.0,
                KeyboardInput(Pressed, _, Some(S)) => delta_view.position[1] -= 1.0,
                KeyboardInput(Pressed, _, Some(E)) => delta_view.position[0] -= 1.0,
                KeyboardInput(Pressed, _, Some(Q)) => delta_view.position[0] += 1.0,
                MouseMoved(new_x, new_y) => {
                    match previous_mouse {
                        Some((prev_x, prev_y)) => {
                            delta_view.direction[0] += (new_x - prev_x) as f32 / 100.0;
                            delta_view.direction[1] += (new_y - prev_y) as f32 / 100.0;
                        },
                        _ => ()
                    }
                    previous_mouse = Some((new_x, new_y));
                },
                _ => ()
            }

            view = view.combine(&delta_view);
        }
    }
}

fn create_display() -> GlutinFacade {
    //let monitor = glium::glutin::get_primary_monitor();
    return WindowBuilder::new()
        //.with_fullscreen(monitor)
        .build_glium().unwrap();
}

fn draw(display: &GlutinFacade, shape: &Shape, run_time: &TimeDuration, view: &View) {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

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
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let mut frame = display.draw();
    let dimensions = frame.get_dimensions();

    frame.clear_color(0.0, 0.0, 0.0, 0.0);

    frame.draw(
        &VertexBuffer::new(display, &shape.vertices).unwrap(),
        &IndexBuffer::new(display, PrimitiveType::TrianglesList, &shape.indices.iter().map(|&i| i - 1).collect::<Vec<_>>()).unwrap(),
        &Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        &uniform! {
            view: Matrices::view(view),
            model: Matrices::rotate(run_time.period(1.2)),
            perspective: Matrices::perspective(dimensions)
        },
        &Default::default()
    ).unwrap();

    frame.finish().unwrap();
}
