use crate::characteristic::Characteristic;
use crate::identities::{Identity, Zero};
use crate::inverse::Inverse;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// R\[x\]/(x^2 + 1) の元。
/// ここでRは型```T```の対象のなす環。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> Complex<T> {
    /// コンストラクタ。1つめの引数が定数項。2つめが1次の項。3つめと4つめが割るイデアルの生成元の1次の係数と定数項。
    pub fn new(real: T, imaginary: T) -> Self {
        Self {
            real: real,
            imaginary: imaginary,
        }
    }
}

/// ```println!```などで見やすく表示させるため、```Display```トレイトを実装。
/// 型```T```がそもそも```Display```トレイトを実装していることを要求。
impl<T: fmt::Display + Zero + Eq> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imaginary == T::zero() {
            write!(f, "{}", self.real)
        } else if self.real == T::zero() {
            write!(f, "{}i", self.imaginary)
        } else {
            write!(f, "({} + {}i)", self.real, self.imaginary)
        }
    }
}

/// 足し算の実装。
/// これら演算は、bやcが異なっている場合（すなわち異なる環の元どうしでの演算を試みた場合）、
/// R\[x\] / (x^2) の元 0 を返すことにする。
impl<T: Copy + Add<Output = T> + Eq> Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl<T: Copy + Add<Output = T> + Eq> AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl<T: Copy + Sub<Output = T> + Eq> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl<T: Copy + Sub<Output = T> + Eq> SubAssign for Complex<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Eq> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.imaginary * rhs.real + self.real * rhs.imaginary,
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Eq> MulAssign for Complex<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.imaginary * other.real + self.real * other.imaginary,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Complex<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl<T: Copy + Zero> Zero for Complex<T> {
    fn zero() -> Self {
        Self {
            real: T::zero(),
            imaginary: T::zero(),
        }
    }
}

impl<T: Copy + Zero + Identity> Identity for Complex<T> {
    fn identity() -> Self {
        Self {
            real: T::identity(),
            imaginary: T::zero(),
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Eq + Zero + Identity>
    Complex<T>
{
    /// 繰り返し二乗法によるべき乗の計算
    pub fn modpow(&self, n: u64) -> Self {
        let mut res_r = T::identity();
        let mut res_i = T::zero();
        let mut a = self.real;
        let mut b = self.imaginary;
        let mut m = n;
        loop {
            if m == 0 {
                break;
            }
            if m % 2 == 1 {
                res_r = res_r * a - res_i * b;
                res_i = res_r * b - a * res_i;
            }
            a = a * a - b * b;
            b = a * b + b * a;
            m = m / 2;
        }
        Self {
            real: res_r,
            imaginary: res_i,
        }
    }
}

impl<T: Characteristic> Characteristic for Complex<T> {
    fn characteristic() -> u64 {
        T::characteristic()
    }
}

impl<
        T: Characteristic
            + Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Sub<Output = T>
            + Eq
            + Zero
            + Identity,
    > Inverse for Complex<T>
{
    fn inverse(self) -> Option<Complex<T>> {
        if self.real == T::zero() && self.imaginary == T::zero() {
            None
        } else {
            Some(self.modpow(T::characteristic() * T::characteristic() - 2))
        }
    }
}
