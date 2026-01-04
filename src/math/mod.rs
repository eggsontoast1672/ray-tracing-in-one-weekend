pub mod interval;
pub mod ray;

/// A 3D vector with three floating-point components.
///
/// This struct represents a vector in 3D space, commonly used in raytracing
/// for positions, directions, colors, and other 3D quantities. It stores
/// the components in an array of three `f64` values.
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// The zero vector.
    ///
    /// This vector has all of its components set to zero. It is the identity value with respect to
    /// vector addition, so all vectors are invariant under addition by the zero vector.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    /// Creates a new vector with specified components.
    ///
    /// Takes three `f64` values representing the x, y, and z components
    /// and returns a new `Vec3` instance. This is the primary constructor
    /// for creating vectors with specific values.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// Generate a vector with random direction and magnitude.
    pub fn random() -> Self {
        Vec3::new(
            crate::random_f64(),
            crate::random_f64(),
            crate::random_f64(),
        )
    }

    /// Generate a vector with random direction and magniture whose components are guaranteed to
    /// lie betweem the bounds supplied.
    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            crate::random_f64_range(min, max),
            crate::random_f64_range(min, max),
            crate::random_f64_range(min, max),
        )
    }

    /// Generate a unit vector with random direction.
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                break p / lensq.sqrt();
            }
        }
    }

    /// Generate a unit vector in the hemisphere of the unit sphere defined by the given normal
    /// vector. In other words, generate a unit vector whose dot product with the given normal
    /// vector is at least 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::Vec3;
    ///
    /// let normal = Vec3::new(0.0, 1.0, 0.0);
    /// let vector = Vec3::random_on_hemisphere(normal);
    ///
    /// assert!(vector.dot(normal) >= 0.0);
    /// ```
    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
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
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Compute the dot product of two vectors.
    ///
    /// This method computes the dot product between this vector and another.
    /// Geometrically, the dot product is the scalar product of the lengths of
    /// the vectors and the cosine of the angle between them.
    pub const fn dot(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Compute the cross product of two vectors.
    ///
    /// This method computes the cross product between this vector and another.
    /// Geometrically, the cross product is the signed area of the parallelogram
    /// formed by the two vectors. The sign of the area depends on the right
    /// hand rule.
    pub const fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Normalize the vector.
    ///
    /// This method returns a unit vector whose direction is the same as this
    /// vector. It is equivalent to this vector scaled down by its length.
    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    /// Return true if this vector is sufficiently close to the zero vector, i.e. all components
    /// are within some small `epsilon` of zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::Vec3;
    ///
    /// const SMALL: f64 = 1e-160;
    ///
    /// assert!(Vec3::new(0.0, 0.0, 0.0).is_near_zero());
    /// assert!(Vec3::new(SMALL, SMALL, SMALL).is_near_zero());
    /// assert!(!Vec3::new(1.0, 2.0, 3.0).is_near_zero());
    /// ```
    pub fn is_near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;

        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

/// Type alias for a 3D point.
pub type Point3 = Vec3;

/// Reflect a vector specularly about a normal vector.
pub fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - normal * 2.0 * vector.dot(normal)
}
