use std::ops;
use std::f32;
use super::deg2rad;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

// indices
impl ops::Index<usize> for Vec2 {
    type Output = f32;

    fn index<'a>(&'a self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!()
        }
    }
}

impl ops::IndexMut<usize> for Vec2 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!()
        }
    }
}

// add ops
impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = self.clone() + other;
    }
}

// sub ops
impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = self.clone() - other;
    }
}

// mul ops
impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, real: f32) -> Vec2 {
        Vec2 {
            x: self.x * real,
            y: self.y * real
        }
    }
}

impl ops::MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, other: Vec2) {
        *self = self.clone() * other;
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, other: f32) {
        *self = self.clone() * other;
    }
}

// div ops
impl ops::Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, real: f32) -> Vec2 {
        Vec2 {
            x: self.x / real,
            y: self.y / real
        }
    }
}

impl ops::DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, other: Vec2) {
        *self = self.clone() / other;
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, other: f32) {
        *self = self.clone() / other;
    }
}

// other functions
impl Vec2 {
    pub fn lenght(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalized(&self) -> Vec2 {
        self.clone() / self.clone().lenght()
    }

    pub fn distance_to(&self, other: Vec2) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn rotate(&self, degrees: f32) -> Vec2 {
        let rad = deg2rad(degrees);
        let a_cos = rad.cos();
        let a_sin = rad.sin();
        Vec2 {
            x: self.x * a_cos - self.y * a_sin,
            y: self.x * a_sin + self.y * a_cos
        }
    }
}