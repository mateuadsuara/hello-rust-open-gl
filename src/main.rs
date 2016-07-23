#[macro_use]

extern crate glium;
use glium::{DisplayBuild, Surface, Program, VertexBuffer, IndexBuffer};
use glium::index::{PrimitiveType};
use glium::backend::glutin_backend::{GlutinFacade};
use glium::glutin::{WindowBuilder, Event, ElementState, VirtualKeyCode};

extern crate time;
use time::{Timespec, Duration};

fn main() {
    let display = create_display();
    let start_time = TimeMeasure::start();

    loop {
        draw(&display, &cube(), &start_time.end());

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

fn draw(display: &GlutinFacade, shape: &Shape, run_time: &TimeDuration) {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        uniform mat4 perspective;
        uniform mat4 matrix;

        void main() {
            gl_Position = perspective * matrix * vec4(position, 1.0);
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
            matrix: rotate(&run_time),
            perspective: perspective(dimensions)
        },
        &Default::default()
    ).unwrap();

    frame.finish().unwrap();
}

type Matrix = [[f32; 4]; 4];

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
