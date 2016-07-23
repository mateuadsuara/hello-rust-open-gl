#[macro_use]

extern crate glium;
use glium::{DisplayBuild, Surface, Program, VertexBuffer, IndexBuffer};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::glutin::{WindowBuilder, Event, ElementState, VirtualKeyCode};
use glium::glutin::Event::*;
use glium::glutin::ElementState::*;
use glium::glutin::VirtualKeyCode::*;

extern crate time;
use time::{Timespec, Duration};

fn main() {
    let display = create_display();
    let start_time = TimeMeasure::start();

    let mut view = View {position: [6.0, 0.0, 0.0], direction: [-2.0, 0.0, 1.0], up: [0.0, 1.0, 0.0]};
    let mut previous_mouse = None;

    loop {
        draw(&display, &cube(), &start_time.end(), &view);

        let mut delta_view = View {position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, 0.0], up: [0.0, 0.0, 0.0]};
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
            println!("{}", view.to_string());
        }
    }
}

struct View {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3]
}

impl View {
    fn combine(&self, b: &View) -> View {
        View {
            position: [self.position[0] + b.position[0], self.position[1] + b.position[1], self.position[2] + b.position[2]],
            direction: [self.direction[0] + b.direction[0], self.direction[1] + b.direction[1], self.direction[2] + b.direction[2]],
            up: [self.up[0] + b.up[0], self.up[1] + b.up[1], self.up[2] + b.up[2]]
        }
    }

    fn to_string(&self) -> String {
        format!("{}, {}, {} / {}, {}, {} / {}, {}, {}",
                self.position[0], self.position[1], self.position[2],
                self.direction[0], self.direction[1], self.direction[2],
                self.up[0], self.up[1], self.up[2])
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
            1, 2, 3,
            2, 3, 4
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
            view: view_matrix(view),
            model: rotate(&run_time),
            perspective: perspective(dimensions)
        },
        &Default::default()
    ).unwrap();

    frame.finish().unwrap();
}

type Matrix = [[f32; 4]; 4];

fn view_matrix(view: &View) -> Matrix {
    let position = view.position;
    let direction = view.direction;
    let up = view.up;

    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
    up[2] * f[0] - up[0] * f[2],
    up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
    f[2] * s_norm[0] - f[0] * s_norm[2],
    f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
    -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
    -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn id() -> Matrix {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn rotate(run_time: &TimeDuration) -> Matrix {
    let t = run_time.period(1.2) * std::f32::consts::PI * 2.0;
    [
        [ t.cos(), t.sin(), 0.0, 0.0],
        [-t.sin(), t.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}

fn perspective(dimensions: (u32, u32)) -> Matrix {
    let (width, height) = dimensions;
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = std::f32::consts::PI / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}

struct TimeMeasure {
    start_time: Timespec,
}

struct TimeDuration {
    time: Duration
}

impl TimeMeasure {
    fn start() -> TimeMeasure {
        TimeMeasure {
            start_time: time::get_time()
        }
    }

    fn end(&self) -> TimeDuration {
        TimeDuration {
            time: time::get_time() - self.start_time
        }
    }
}

impl TimeDuration {
    fn period(&self, period_time_in_seconds: f32) -> f32 {
        return (((self.time.num_milliseconds() as f32) / 1000.0) % period_time_in_seconds ) / period_time_in_seconds;
    }
}
