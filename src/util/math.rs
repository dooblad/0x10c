use cgmath;
use cgmath::{EuclideanSpace, InnerSpace, SquareMatrix};
use gl::types::GLfloat;


// We're pretty much always going to be working with OpenGL floats, so we might as well
// define some convenience types.
pub type Point3 = cgmath::Point3<GLfloat>;
pub type Vector2 = cgmath::Vector2<GLfloat>;
pub type Vector3 = cgmath::Vector3<GLfloat>;
pub type Matrix4 = cgmath::Matrix4<GLfloat>;


/// Stores matrices that define translation, rotation, and scaling.
pub struct Transformation {
    /// Position for each axis.
    translation: Vector3,
    /// Rotation angle for each axis.
    rotation: Vector3,
    /// Scale factor for each axis.
    scale: Vector3,
    /// A cached model matrix to avoid unnecessary multiplication when the transformation
    /// hasn't changed.
    cached: Option<Matrix4>,
}

impl Transformation {
    pub fn new() -> Transformation {
        Transformation {
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            cached: None,
        }
    }

    /// Produces a matrix that transforms from model space to world space.
    pub fn to_model_matrix(&mut self) -> Matrix4 {
        match self.cached {
            Some(c) => c.clone(),
            None => {
                let mut t_mat = Matrix4::identity();
                t_mat[3][0] = self.translation[0];
                t_mat[3][1] = self.translation[1];
                t_mat[3][2] = self.translation[2];

                let mut x_rot_mat = Matrix4::identity();
                {
                    let x_rot = self.rotation[0];
                    x_rot_mat[1][1] = x_rot.cos();
                    x_rot_mat[2][1] = -x_rot.sin();
                    x_rot_mat[1][2] = x_rot.sin();
                    x_rot_mat[2][2] = x_rot.cos();
                }

                let mut y_rot_mat = Matrix4::identity();
                {
                    let y_rot = self.rotation[1];
                    y_rot_mat[0][0] = y_rot.cos();
                    y_rot_mat[2][0] = y_rot.sin();
                    y_rot_mat[0][2] = -y_rot.sin();
                    y_rot_mat[2][2] = y_rot.cos();
                }

                let mut z_rot_mat = Matrix4::identity();
                {
                    let z_rot = self.rotation[2];
                    z_rot_mat[0][0] = z_rot.cos();
                    z_rot_mat[1][0] = -z_rot.sin();
                    z_rot_mat[0][1] = z_rot.sin();
                    z_rot_mat[1][1] = z_rot.cos();
                }

                let mut scale_mat = Matrix4::identity();
                for i in 0..3 {
                    scale_mat[i][i] = self.scale[i];
                }

                let model = t_mat * x_rot_mat * y_rot_mat * z_rot_mat * scale_mat;
                self.cached = Some(model.clone());
                model
            },
        }
    }

    /// Translates to `position`.  Note this translation is absolute and not relative to
    /// the mesh's current position.
    pub fn move_to(&mut self, position: Point3) {
        self.translation = position.to_vec();
        self.cached = None;
    }

    /// Translates by `position`.  That is, `position` is added to the current position.
    pub fn translate(&mut self, position: Vector3) {
        self.translation += position;
        self.cached = None;
    }

    /// Rotates the i'th axis by a magnitude equal to the i'th component of `axes`.
    pub fn rotate(&mut self, axes: Vector3) {
        self.rotation += axes;
        self.cached = None;
    }

    /// Scales the i'th axis (multiplicatively) by a magnitude equal to the i'th component
    /// of `axes`.
    pub fn scale(&mut self, axes: Vector3) {
        for i in 0..3 {
            self.scale[i] *= axes[i];
        }
        self.cached = None;
    }
}


/// A form of rotation defined entirely by a horizontal and vertical angle.
pub struct Rotation {
    pub horizontal_angle: f32,
    pub vertical_angle: f32,
}

impl Rotation {
    pub fn to_view_vec(&self) -> Vector3 {
        use std;

        // TBH, I'm not really sure why we need to add PI here, but this makes it so the
        // coordinate system is normal with zeroes for both horizontal and vertical
        // angles.
        let horizontal_angle = self.horizontal_angle + std::f32::consts::PI;
        Vector3 {
            x: self.vertical_angle.cos() * horizontal_angle.sin(),
            y: self.vertical_angle.sin(),
            z: self.vertical_angle.cos() * horizontal_angle.cos(),
        }.normalize()
    }
}

impl Clone for Rotation {
    fn clone(&self) -> Rotation {
        Rotation {
            horizontal_angle: self.horizontal_angle,
            vertical_angle: self.vertical_angle,
        }
    }

    fn clone_from(&mut self, source: &Rotation) {
        self.horizontal_angle = source.horizontal_angle;
        self.vertical_angle = source.vertical_angle;
    }
}

#[derive(Debug)]
pub struct Ray3 {
    pub pos: Point3,
    pub dir: Vector3,
}
