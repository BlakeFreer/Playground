use crate::Vector;

/// Specialized 3D vector
pub type Vector3 = Vector<3>;
impl Vector3 {
    /// Get the `x` coordinate.
    /// ```
    /// # use riesz::Vector;
    /// let v1 = Vector::new([1.0, 2.0, 3.0]);
    /// assert_eq!(v1.x(), 1.0);
    /// ```
    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn set_x(&mut self, x: f64) {
        self[0] = x;
    }

    /// Get the `y` coordinate.
    /// ```
    /// # use riesz::Vector;
    /// let v1 = Vector::new([1.0, 2.0, 3.0]);
    /// assert_eq!(v1.y(), 2.0);
    /// ```
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn set_y(&mut self, y: f64) {
        self[0] = y;
    }

    /// Get the `z` coordinate.
    /// ```
    /// # use riesz::Vector;
    /// let v1 = Vector::new([1.0, 2.0, 3.0]);
    /// assert_eq!(v1.z(), 3.0);
    /// ```
    pub fn z(&self) -> f64 {
        self[2]
    }
    pub fn set_z(&mut self, z: f64) {
        self[0] = z;
    }

    /// Compute the cross product of two `Vector<3>`
    /// ```
    /// # use riesz::Vector;
    /// let X = Vector::new([1.0, 0.0, 0.0]);
    /// let Y = Vector::new([0.0, 1.0, 0.0]);
    /// let Z = Vector::new([0.0, 0.0, 1.0]);
    /// assert_eq!(Vector::cross(X, Y), Z);
    /// ```
    pub fn cross(v1: Self, v2: Self) -> Self {
        Self::new([
            v1.y() * v2.z() - v1.z() * v2.y(),
            -v1.x() * v2.z() + v1.z() * v2.x(),
            v1.x() * v2.y() - v1.y() * v2.x(),
        ])
    }
}
