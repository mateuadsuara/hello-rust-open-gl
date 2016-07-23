extern crate std;
use self::std::f32::consts::{PI};

pub type Matrix = [[f32; 4]; 4];

pub struct Matrices {
}

impl Matrices {
    //pub fn id() -> Matrix {
    //    [
    //        [1.0, 0.0, 0.0, 0.0],
    //        [0.0, 1.0, 0.0, 0.0],
    //        [0.0, 0.0, 1.0, 0.0],
    //        [0.0, 0.0, 0.0, 1.0f32],
    //    ]
    //}

    pub fn view(view: &View) -> Matrix {
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

    pub fn perspective(dimensions: (u32, u32)) -> Matrix {
        let (width, height) = dimensions;
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = PI / 3.0;
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

    pub fn rotate(percentage: f32) -> Matrix {
        let t = percentage * PI * 2.0;
        [
            [ t.cos(), t.sin(), 0.0, 0.0],
            [-t.sin(), t.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    }
}


pub type ThreeD = [f32; 3];

pub struct View {
    pub position: ThreeD,
    pub direction: ThreeD,
    pub up: ThreeD
}

impl View {
    pub fn new(position: ThreeD, direction: ThreeD, up: ThreeD) -> View {
        View {position: position, direction: direction, up: up}
    }

    pub fn neutral() -> View {
        View {position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, 0.0], up: [0.0, 0.0, 0.0]}
    }

    pub fn combine(&self, b: &View) -> View {
        View {
            position: [self.position[0] + b.position[0], self.position[1] + b.position[1], self.position[2] + b.position[2]],
            direction: [self.direction[0] + b.direction[0], self.direction[1] + b.direction[1], self.direction[2] + b.direction[2]],
            up: [self.up[0] + b.up[0], self.up[1] + b.up[1], self.up[2] + b.up[2]]
        }
    }

   //pub fn to_string(&self) -> String {
   //    format!("{}, {}, {} / {}, {}, {} / {}, {}, {}",
   //            self.position[0], self.position[1], self.position[2],
   //            self.direction[0], self.direction[1], self.direction[2],
   //            self.up[0], self.up[1], self.up[2])
   //}
}

