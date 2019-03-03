use std::ops;
use std::f32;
use super::deg2rad;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

// add ops
impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = self.clone() + other;
    }
}

// sub ops
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = self.clone() - other;
    }
}

// mul ops
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, real: f32) -> Vec3 {
        Vec3 {
            x: self.x * real,
            y: self.y * real,
            z: self.z * real
        }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = self.clone() * other;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = self.clone() * other;
    }
}

// div ops
impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, real: f32) -> Vec3 {
        Vec3 {
            x: self.x / real,
            y: self.y / real,
            z: self.z / real
        }
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = self.clone() / other;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = self.clone() / other;
    }
}

// other functions
impl Vec3 {
    pub fn lenght(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn normalized(&self) -> Vec3 {
        self.clone() / self.clone().lenght()
    }

    pub fn distance_to(&self, other: Vec3) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    // todo (if needed)
    pub fn rotate_x(&self, degrees: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn rotate_y(&self, degrees: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn rotate_z(&self, degrees: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}