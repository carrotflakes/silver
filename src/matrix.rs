//! Affine transformation

use crate::vec3::Vec3;

/// Matrix for affine transformation.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(pub [f64; 12]);

impl Matrix {
    /// Create a Matrix that no transform.
    pub fn new() -> Self {
        Matrix([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
        ])
    }

    /// Create new Matrix translated with specified position from myself.
    pub fn translate(&self, v: &Vec3) -> Self {
        let s = &self.0;
        Matrix([
            s[0], s[1], s[2], s[3] + v.x(),
            s[4], s[5], s[6], s[7] + v.y(),
            s[8], s[9], s[10], s[11] + v.z(),
        ])
    }

    /// Create new Matrix scaled with specified size from myself.
    pub fn scale(&self, v: &Vec3) -> Self {
        let s = &self.0;
        Matrix([
            s[0] * v.x(), s[1] * v.x(), s[2] * v.x(), s[3] * v.x(),
            s[4] * v.y(), s[5] * v.y(), s[6] * v.y(), s[7] * v.y(),
            s[8] * v.z(), s[9] * v.z(), s[10] * v.z(), s[11] * v.z(),
        ])
    }

    /// Create new Matrix rotated with specified angle from myself.
    pub fn rotate_x(&self, rad: f64) -> Self {
        let s = &self.0;
        let (sin, cos) = rad.sin_cos();
        Matrix([
            s[0],
            s[1] * cos + s[2] * sin,
            -s[1] * sin + s[2] * cos,
            s[3],
            s[4],
            s[5] * cos + s[6] * sin,
            -s[5] * sin + s[6] * cos,
            s[7],
            s[8],
            s[9] * cos + s[10] * sin,
            -s[9] * sin + s[10] * cos,
            s[11],
        ])
    }

    // /// Create new Matrix skewed with specified y-axis amount from myself.
    // pub fn skew_y(&self, dy: f64) -> Self {
    //     let s = &self.0;
    //     Matrix([
    //         s[0],
    //         s[1],
    //         s[2],
    //         s[3] + s[0] * dy,
    //         s[4] + s[1] * dy,
    //         s[5] + s[2] * dy,
    //     ])
    // }

    // /// Create new Matrix skewed with specified x-axis amount from myself.
    // pub fn skew_x(&self, dx: f64) -> Self {
    //     let s = &self.0;
    //     Matrix([
    //         s[0] + s[3] * dx,
    //         s[1] + s[4] * dx,
    //         s[2] + s[5] * dx,
    //         s[3],
    //         s[4],
    //         s[5],
    //     ])
    // }

    /// Transform the [`Vec3`].
    pub fn apply(&self, v: &Vec3) -> Vec3 {
        let s = &self.0;
        Vec3::new([
            v[0] * s[0] + v[1] * s[1] + v[2] * s[2] + s[3],
            v[0] * s[4] + v[1] * s[5] + v[2] * s[6] + s[7],
            v[0] * s[8] + v[1] * s[9] + v[2] * s[10] + s[11],
        ])
    }

    /// Inverse the matrix
    /// Ideally, `matrix.inverse().inverse() == matrix`.
    pub fn inverse(&self) -> Self {
        let s = &self.0;
        // let a = 1.0 / (s[0] * s[4] - s[1] * s[3]);
        // Matrix([
        //     a * s[4],
        //     -a * s[1],
        //     a * (s[1] * s[5] - s[2] * s[4]),
        //     -a * s[3],
        //     a * s[0],
        //     -a * (s[0] * s[5] - s[2] * s[3]),
        // ])
        //  0 1 2
        //  3 4 5
        //  0  1  2  3
        //  4  5  6  7
        //  8  9 10 11
        let a = 1.0 / (s[0] * s[5] * s[10] + s[1] * s[6] * s[8] + s[2] * s[4] * s[9] - s[2] * s[5] * s[8] - s[1] * s[4] * s[10] - s[0] * s[6] * s[9]);
        Matrix([
            a * (s[5] * s[10] - s[6] * s[9]),
            a * (-s[1] * s[10] + s[2] * s[9]),
            a * (s[1] * s[6] - s[2] * s[5]),
            a * (-s[1] * s[6] * s[11] - s[2] * s[7] * s[9] - s[3] * s[5] * s[10] + s[3] * s[6] * s[9] + s[2] * s[5] * s[11] + s[1] * s[7] * s[10]),
            a * (-s[4] * s[10] + s[6] * s[8]),
            a * (s[0] * s[10] - s[2] * s[8]),
            a * (-s[0] * s[6] + s[2] * s[4]),
            a * (s[0] * s[6] * s[11] + s[2] * s[7] * s[8] + s[3] * s[4] * s[10] - s[3] * s[6] * s[8] - s[2] * s[4] * s[11] - s[0] * s[7] * s[10]),
            a * (s[4] * s[9] - s[5] * s[8]),
            a * (-s[0] * s[9] + s[1] * s[8]),
            a * (s[0] * s[5] - s[1] * s[4]),
            a * (-s[0] * s[5] * s[11] - s[1] * s[7] * s[8] - s[3] * s[4] * s[9] + s[3] * s[5] * s[8] + s[1] * s[4] * s[11] + s[0] * s[7] * s[9]),
        ])
    }

    /// Return the multiplication of the two matrices.
    pub fn then(&self, rhs: &Matrix) -> Self {
        let s = &self.0;
        let t = &rhs.0;
        // Matrix([
        //     s[0] * t[0] + s[3] * t[1],
        //     s[1] * t[0] + s[4] * t[1],
        //     s[2] * t[0] + s[5] * t[1] + t[2],
        //     s[0] * t[3] + s[3] * t[4],
        //     s[1] * t[3] + s[4] * t[4],
        //     s[2] * t[3] + s[5] * t[4] + t[5],
        // ])
        Matrix([
            s[0] * t[0] + s[4] * t[1] + s[8] * t[2],
            s[1] * t[0] + s[5] * t[1] + s[9] * t[2],
            s[2] * t[0] + s[6] * t[1] + s[10] * t[2],
            s[3] * t[0] + s[7] * t[1] + s[11] * t[2] + t[3],
            s[0] * t[4] + s[4] * t[5] + s[8] * t[6],
            s[1] * t[4] + s[5] * t[5] + s[9] * t[6],
            s[2] * t[4] + s[6] * t[5] + s[10] * t[6],
            s[3] * t[4] + s[7] * t[5] + s[11] * t[6] + t[7],
            s[0] * t[8] + s[4] * t[9] + s[8] * t[10],
            s[1] * t[8] + s[5] * t[9] + s[9] * t[10],
            s[2] * t[8] + s[6] * t[9] + s[10] * t[10],
            s[3] * t[8] + s[7] * t[9] + s[11] * t[10] + t[11],
        ])
    }

    /// Return whether it is unit matrix.
    pub fn is_unit(&self) -> bool {
        self == &Default::default()
    }

    /// Return whether it is directly or indirectly.
    /// An indirect matrix makes path flip.
    pub fn is_direct(&self) -> bool {
        self.0[1] * self.0[3] <= self.0[0] * self.0[4]
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix::new()
    }
}

#[test]
fn test() {
    // let m = Matrix([2.0, 3.0, 5.0, 0.0, 11.0, 13.0, 17.0, 19.0, 23.0, 29.0, 31.0, 0.0]);
    let m = Matrix([
        1.1, 0.1, 0.3, 3.0, 
        0.5, 1.0, 0.0, 2.0, 
        4.0, 0.2, 1.0, 4.0]);
    // let m = Matrix::new().translate(&Vec3::new([1., 2., 3.])).scale(&Vec3::new([2., 2., 2.])).rotate_x(0.1);
    dbg!(&m);
    dbg!(m.inverse().inverse());
    Matrix::new().rotate_x(60.0f64.to_radians());
    // let am = Matrix::new()
    //     .translate(1.0, 2.0)
    //     .rotate(1.0)
    //     .scale(0.5, 0.6);
    // assert!((Point(3.0, 4.0) - am.inverse().apply(am.apply(Point(3.0, 4.0)))).norm() < 0.00001);

    // assert_eq!(
    //     am.rotate(0.1).then(&Matrix::new().translate(-0.5, -0.6)),
    //     am.rotate(0.1).translate(-0.5, -0.6)
    // );
    // assert_eq!(
    //     am.rotate(0.1).then(&Matrix::new().scale(-0.5, -0.6)),
    //     am.rotate(0.1).scale(-0.5, -0.6)
    // );
    // assert_eq!(
    //     am.rotate(0.1).then(&Matrix::new().rotate(0.3)),
    //     am.rotate(0.1).rotate(0.3)
    // );
    // assert!(
    //     am.rotate(0.1)
    //         .then(
    //             &Matrix::new()
    //                 .scale(0.5, 0.6)
    //                 .translate(-0.5, -0.6)
    //                 .rotate(0.3)
    //         )
    //         .0
    //         .iter()
    //         .zip(
    //             am.rotate(0.1)
    //                 .scale(0.5, 0.6)
    //                 .translate(-0.5, -0.6)
    //                 .rotate(0.3)
    //                 .0
    //                 .iter()
    //         )
    //         .map(|(a, b)| (a - b).abs())
    //         .sum::<f64>()
    //         < 0.0001
    // );
    // assert_eq!(Matrix::new().apply((0.0f64, 0.0f64)), (0.0, 0.0));
}