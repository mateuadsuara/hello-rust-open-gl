extern crate cgmath;
use self::cgmath::{Matrix4};

extern crate std;
use self::std::f32::consts::{PI};

pub type Matrix = [[f32; 4]; 4];
pub type ModelTransformation = Matrix4<f32>;

pub struct Mat4 {
    elements: [[f32; 4]; 4]
}

impl Mat4 {
    pub fn id() -> Mat4 {
        Mat4 {
            elements: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        }
    }

    pub fn new() -> Mat4 {
        Mat4::id()
    }

    pub fn scale(&self, vector: [f32; 3]) -> Mat4 {
        let this = self.elements;
        let x = vector[0];
        let y = vector[1];
        let z = vector[2];
        Mat4 {
            elements: [
                [this[0][0] * x, this[0][1] * x, this[0][2] * x, this[0][3] * x],
                [this[1][0] * y, this[1][1] * y, this[1][2] * y, this[1][3] * y],
                [this[2][0] * z, this[2][1] * z, this[2][2] * z, this[2][3] * z],
                [this[3][0],     this[3][1]    , this[3][2]    , this[3][3]    ],
            ]
        }
    }

    pub fn translate(&self, vector: [f32; 3]) -> Mat4 {
        let this = self.elements;
        let x = vector[0];
        let y = vector[1];
        let z = vector[2];
        Mat4 {
            elements: [
                [this[0][0], this[0][1], this[0][2], this[0][3]],
                [this[1][0], this[1][1], this[1][2], this[1][3]],
                [this[2][0], this[2][1], this[2][2], this[2][3]],
                [
                    this[0][0] * x + this[1][0] * y + this[2][0] * z + this[3][0],
                    this[0][1] * x + this[1][1] * y + this[2][1] * z + this[3][1],
                    this[0][2] * x + this[1][2] * y + this[2][2] * z + this[3][2],
                    this[0][3] * x + this[1][3] * y + this[2][3] * z + this[3][3],
                ]
            ]
        }
    }

    pub fn rotate(&self, percentage: f32, vector: [f32; 3]) -> Mat4 {
        Mat4::rotate_angle(self, percentage * PI * 2., vector)
    }

    pub fn rotate_angle(&self, angle: f32, vector: [f32; 3]) -> Mat4 {
        let this = self.elements;
        let _x = vector[0];
        let _y = vector[1];
        let _z = vector[2];

        let len = 1. / (_x * _x + _y * _y + _z * _z).sqrt() as f32;

        let x = _x * len;
        let y = _y * len;
        let z = _z * len;

        let s = angle.sin();
        let c = angle.cos();
        let t = 1. - c;

        let b00 = x * x * t + c;
        let b01 = y * x * t + z * s;
        let b02 = z * x * t - y * s;
        let b10 = x * y * t - z * s;
        let b11 = y * y * t + c;
        let b12 = z * y * t + x * s;
        let b20 = x * z * t + y * s;
        let b21 = y * z * t - x * s;
        let b22 = z * z * t + c;

        Mat4 {
            elements: [
                [
                    this[0][0] * b00 + this[1][0] * b01 + this[2][0] * b02,
                    this[0][1] * b00 + this[1][1] * b01 + this[2][1] * b02,
                    this[0][2] * b00 + this[1][2] * b01 + this[2][2] * b02,
                    this[0][3] * b00 + this[1][3] * b01 + this[2][3] * b02,
                ],
                [
                    this[0][0] * b10 + this[1][0] * b11 + this[2][0] * b12,
                    this[0][1] * b10 + this[1][1] * b11 + this[2][1] * b12,
                    this[0][2] * b10 + this[1][2] * b11 + this[2][2] * b12,
                    this[0][3] * b10 + this[1][3] * b11 + this[2][3] * b12,
                ],
                [
                    this[0][0] * b20 + this[1][0] * b21 + this[2][0] * b22,
                    this[0][1] * b20 + this[1][1] * b21 + this[2][1] * b22,
                    this[0][2] * b20 + this[1][2] * b21 + this[2][2] * b22,
                    this[0][3] * b20 + this[1][3] * b21 + this[2][3] * b22,
                ],
                [
                    this[3][0],
                    this[3][1],
                    this[3][2],
                    this[3][3]
                ],
            ]
        }
    }

    pub fn end(&self) -> [[f32; 4]; 4] {
        self.elements
    }
}

pub struct Matrices {
}

impl Matrices {
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

    pub fn add(&self, b: &View) -> View {
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

