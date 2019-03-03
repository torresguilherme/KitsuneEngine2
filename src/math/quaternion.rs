use std::ops;
use std::f32;
use super::deg2rad;

#[derive(Debug, Clone, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

// add ops
impl ops::Add for Quat {
    type Output = Quat;

    fn add(self, other: Quat) -> Quat {
        Quat {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl ops::AddAssign for Quat {
    fn add_assign(&mut self, other: Quat) {
        *self = self.clone() + other;
    }
}

// sub ops
impl ops::Sub for Quat {
    type Output = Quat;

    fn sub(self, other: Quat) -> Quat {
        Quat {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl ops::SubAssign for Quat {
    fn sub_assign(&mut self, other: Quat) {
        *self = self.clone() - other;
    }
}

// mul ops
impl ops::Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, other: Quat) -> Quat {
        Quat {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w
        }
    }
}

impl ops::Mul<f32> for Quat {
    type Output = Quat;

    fn mul(self, real: f32) -> Quat {
        Quat {
            x: self.x * real,
            y: self.y * real,
            z: self.z * real,
            w: self.w * real
        }
    }
}

impl ops::MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, other: Quat) {
        *self = self.clone() * other;
    }
}

impl ops::MulAssign<f32> for Quat {
    fn mul_assign(&mut self, other: f32) {
        *self = self.clone() * other;
    }
}

// div ops
impl ops::Div<Quat> for Quat {
    type Output = Quat;

    fn div(self, other: Quat) -> Quat {
        Quat {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w
        }
    }
}

impl ops::Div<f32> for Quat {
    type Output = Quat;

    fn div(self, real: f32) -> Quat {
        Quat {
            x: self.x / real,
            y: self.y / real,
            z: self.z / real,
            w: self.w / real
        }
    }
}

impl ops::DivAssign<Quat> for Quat {
    fn div_assign(&mut self, other: Quat) {
        *self = self.clone() / other;
    }
}

impl ops::DivAssign<f32> for Quat {
    fn div_assign(&mut self, other: f32) {
        *self = self.clone() / other;
    }
}

// other functions
impl Quat {
    pub fn lenght(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Quat {
        self.clone() / self.clone().lenght()
    }

    pub fn conjugated(&self) -> Quat {
        Quat {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
    }
}