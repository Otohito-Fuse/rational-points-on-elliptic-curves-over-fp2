use crate::identities::Identity;
use crate::inverse::Inverse;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RationalPoint<T> {
    Point(T, T),
    O,
}

impl<T> RationalPoint<T> {
    pub fn new(x: T, y: T) -> Self {
        RationalPoint::Point(x, y)
    }
}

impl<T: fmt::Display> fmt::Display for RationalPoint<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RationalPoint::O => write!(f, "O"),
            RationalPoint::Point(x, y) => write!(f, "({}, {})", x, y),
        }
    }
}

/// 本当は演算子のオーバーロードをしたかったが、係数aをstaticに用意するのが無理だったので断念。これは記録です。
impl<
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy + Eq + Inverse + Identity + A,
    > Add for RationalPoint<T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match self {
            RationalPoint::O => rhs,
            RationalPoint::Point(x1, y1) => match rhs {
                RationalPoint::O => RationalPoint::Point(x1, y1),
                RationalPoint::Point(x2, y2) => {
                    if x1 == x2 {
                        if y1 != y2 {
                            RationalPoint::O
                        } else {
                            let id = T::identity();
                            let m = ((id + id + id) * x1 * x1 + T::a())
                                * ((id + id) * y1).inverse().unwrap();
                            RationalPoint::Point(m * m - x1 - x1, m * (x1 - m * m + x1 + x1) - y1)
                        }
                    } else {
                        let m = (y2 - y1) * ((x2 - x1).inverse().unwrap());
                        RationalPoint::Point(m * m - x1 - x2, m * (x1 - m * m + x1 + x2) - y1)
                    }
                }
            },
        }
    }
}

impl<T: Neg<Output = T>> Neg for RationalPoint<T> {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            RationalPoint::O => self,
            RationalPoint::Point(x, y) => RationalPoint::Point(x, -y),
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy + Eq + Inverse + Identity>
    RationalPoint<T>
{
    /// 有理点の足し算。aは y^2 = x^3 + ax + b の a。
    pub fn add_rational_points(&self, rhs: &Self, a: T) -> Self {
        match *self {
            RationalPoint::O => *rhs,
            RationalPoint::Point(x1, y1) => match *rhs {
                RationalPoint::O => RationalPoint::Point(x1, y1),
                RationalPoint::Point(x2, y2) => {
                    if x1 == x2 {
                        if y1 != y2 {
                            RationalPoint::O
                        } else {
                            let id = T::identity();
                            let m = ((id + id + id) * x1 * x1 + a)
                                * ((id + id) * y1).inverse().unwrap();
                            RationalPoint::Point(m * m - x1 - x1, m * (x1 - m * m + x1 + x1) - y1)
                        }
                    } else {
                        let m = (y2 - y1) * ((x2 - x1).inverse().unwrap());
                        RationalPoint::Point(m * m - x1 - x2, m * (x1 - m * m + x1 + x2) - y1)
                    }
                }
            },
        }
    }
}

pub trait A {
    fn a() -> Self;
}
