use cgmath;
use std;

pub struct Camera {
    pub view_matrix: cgmath::Matrix4<f32>,
    pub projection_matrix: cgmath::Matrix4<f32>,
    pub position: cgmath::Vector3<f32>,
    pub horizontal_angle: f32,
    pub vertical_angle: f32,
}

impl Camera {
    // TODO: This operates under the assumption that the window size never changes.  Make it update
    // somehow.
    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            view_matrix: view_matrix(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]),
            projection_matrix: projection_matrix(width, height),
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            horizontal_angle: 0.0,
            vertical_angle: 0.0,
        }
    }
}

fn projection_matrix(width: u32, height: u32) -> cgmath::Matrix4<f32> {
    // TODO: Is this shit backwards?
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = std::f32::consts::PI / 3.0;

    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    cgmath::Matrix4::from([
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
        [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
    ])
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> cgmath::Matrix4<f32> {
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

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]
    ];

    cgmath::Matrix4::from([
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ])
}
