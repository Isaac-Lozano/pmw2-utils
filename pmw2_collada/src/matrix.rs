pub struct Matrix(pub [f32; 16]);

impl Matrix {
    pub fn new() -> Matrix {
        Matrix(
            [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        )
    }

    pub fn mult(self, other: &Matrix) -> Matrix {
        let mut out = Matrix::new();

        for row in 0..4 {
            for col in 0..4 {
                out.0[row * 4 + col] =
                    self.0[row * 4 + 0] * other.0[(0 * 4) + col] +
                    self.0[row * 4 + 1] * other.0[(1 * 4) + col] +
                    self.0[row * 4 + 2] * other.0[(2 * 4) + col] +
                    self.0[row * 4 + 3] * other.0[(3 * 4) + col];
            }
        }

        out
    }

    pub fn rot_x(self, angle: f32) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();
        let rot = Matrix (
            [
                1.0, 0.0,  0.0, 0.0,
                0.0, cos, -sin, 0.0,
                0.0, sin,  cos, 0.0,
                0.0, 0.0,  0.0, 1.0,
            ]
        );
        self.mult(&rot)
    }

    pub fn rot_y(self, angle: f32) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();
        let rot = Matrix (
            [
                 cos, 0.0, sin, 0.0,
                 0.0, 1.0, 0.0, 0.0,
                -sin, 0.0, cos, 0.0,
                 0.0, 0.0, 0.0, 1.0,
            ]
        );
        self.mult(&rot)
    }

    pub fn rot_z(self, angle: f32) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();
        let rot = Matrix (
            [
                cos, -sin, 0.0, 0.0,
                sin,  cos, 0.0, 0.0,
                0.0,  0.0, 1.0, 0.0,
                0.0,  0.0, 0.0, 1.0,
            ]
        );
        self.mult(&rot)
    }

    pub fn translate(mut self, val: (f32, f32, f32, f32)) -> Matrix {
        self.0[3] += val.0;
        self.0[7] += val.1;
        self.0[11] += val.2;
        self.0[15] += val.3;
        self
    }

    pub fn scale(mut self, val: (f32, f32, f32)) -> Matrix {
        self.0[0] *= val.0;
        self.0[5] *= val.1;
        self.0[10] *= val.2;
        self
    }

    pub fn rot_yxz(mut self, val: (f32, f32, f32)) -> Matrix {
        self = self.rot_z(val.2);
        self = self.rot_x(val.0);
        self = self.rot_y(val.1);
        self
    }
}