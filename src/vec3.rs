/// A 3D vector with three floating-point components.
///
/// This struct represents a vector in 3D space, commonly used in raytracing
/// for positions, directions, colors, and other 3D quantities. It stores
/// the components in an array of three `f64` values.
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    /// Creates a new zero vector.
    ///
    /// Returns a `Vec3` instance with all components (x, y, z) set to 0.0.
    /// This is useful for initializing vectors that will be accumulated or
    /// used as a starting point for computations.
    pub const fn zero() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    /// Creates a new vector with specified components.
    ///
    /// Takes three `f64` values representing the x, y, and z components
    /// and returns a new `Vec3` instance. This is the primary constructor
    /// for creating vectors with specific values.
    pub const fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    /// Returns the x-component of the vector.
    ///
    /// Provides read-only access to the first component of the vector.
    pub const fn x(&self) -> f64 {
        self.e[0]
    }

    /// Returns the y-component of the vector.
    ///
    /// Provides read-only access to the second component of the vector.
    pub const fn y(&self) -> f64 {
        self.e[1]
    }

    /// Returns the z-component of the vector.
    ///
    /// Provides read-only access to the third component of the vector.
    pub const fn z(&self) -> f64 {
        self.e[2]
    }

    /// Computes the Euclidean length of the vector.
    ///
    /// Returns the square root of the sum of the squares of the components.
    /// This is the magnitude or norm of the vector in 3D space.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Computes the squared Euclidean length of the vector.
    ///
    /// Returns the sum of the squares of the x, y, and z components.
    /// This is useful for length comparisons without the cost of computing
    /// the square root, as it avoids the `sqrt` operation.
    pub const fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// Compute the dot product of two vectors.
    ///
    /// This method computes the dot product between this vector and another.
    /// Geometrically, the dot product is the scalar product of the lengths of
    /// the vectors and the cosine of the angle between them.
    pub const fn dot(self, rhs: Vec3) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    /// Compute the cross product of two vectors.
    ///
    /// This method computes the cross product between this vector and another.
    /// Geometrically, the cross product is the signed area of the parallelogram
    /// formed by the two vectors. The sign of the area depends on the right
    /// hand rule.
    pub const fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }

    /// Normalize the vector.
    ///
    /// This method returns a unit vector whose direction is the same as this
    /// vector. It is equivalent to this vector scaled down by its length.
    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

/// Type alias for a 3D point.
pub type Point3 = Vec3;
