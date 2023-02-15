use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Mul, Neg, Sub},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! vec2 {
    () => {
        $crate::vec::Vec2::new(0., 0.)
    };
    ($val:expr) => {
        $crate::vec::Vec2::new($val, $val)
    };
    ($x:expr, $y:expr) => {
        $crate::vec::Vec2::new($x, $y)
    };
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Vec2 {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Vec2 {
    pub fn rotated(&self, angle: f64, origin: Self) -> Self {
        let (sin, cos) = angle.to_radians().sin_cos();
        let translated = self - origin;
        let rotated = Self::new(
            cos * translated.x - sin * translated.y,
            sin * translated.x + cos * translated.y,
        );
        rotated + origin
    }

    pub fn halfway_point(&self, other: &Self) -> Self {
        (self + other) * 0.5
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match f.precision() {
            Some(precision) => write!(
                f,
                "{} {}",
                format!("{:.*}", precision, self.x)
                    .trim_end_matches('0')
                    .trim_end_matches('.'),
                format!("{:.*}", precision, self.y)
                    .trim_end_matches('0')
                    .trim_end_matches('.'),
            ),
            None => write!(f, "{} {}", self.x, self.y),
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

macro_rules! bin_impl {
    ($trait:ident, $lhs:ty, $rhs:ty, $method_name:ident, $op:tt) => {
        impl $trait<$rhs> for $lhs {
            type Output = Vec2;

            fn $method_name(self, rhs: $rhs) -> Self::Output {
                Vec2 {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                }
            }
        }
    };
}

bin_impl!(Add,  Vec2,  Vec2, add, +);
bin_impl!(Add, &Vec2,  Vec2, add, +);
bin_impl!(Add,  Vec2, &Vec2, add, +);
bin_impl!(Add, &Vec2, &Vec2, add, +);

bin_impl!(Sub,  Vec2,  Vec2, sub, -);
bin_impl!(Sub, &Vec2,  Vec2, sub, -);
bin_impl!(Sub,  Vec2, &Vec2, sub, -);
bin_impl!(Sub, &Vec2, &Vec2, sub, -);

impl Mul for Vec2 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs.mul(self)
    }
}
