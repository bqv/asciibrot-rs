
extern crate num;

use std::{ops, clone, fmt, cmp, hash};

#[derive(Clone, Copy, PartialEq)]
pub struct Quaternion<T> {
    pub x: T,
    pub i: T,
    pub j: T,
    pub k: T
}

impl<T: num::Num + clone::Clone> ops::Add for Quaternion<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Quaternion { x: self.x + other.x, i: self.i + other.i, j: self.j + other.j, k: self.k + other.k }
    }
}

impl<T: num::Num + clone::Clone> num::Zero for Quaternion<T> {
    #[inline]
    fn zero() -> Self {
        Quaternion { x: T::zero(), i: T::zero(), j: T::zero(), k: T::zero() }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.i == T::zero() && self.j == T::zero() && self.k == T::zero()
    }
}

impl<T: num::Num + clone::Clone> ops::Mul for Quaternion<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        let x = (self.x.clone()*other.x.clone()) - (self.i.clone()*other.i.clone())
            - (self.j.clone()*other.j.clone()) - (self.k.clone()*other.k.clone());
        let i = (self.x.clone()*other.i.clone()) + (self.i.clone()*other.x.clone())
            + (self.j.clone()*other.k.clone()) - (self.k.clone()*other.j.clone());
        let j = (self.x.clone()*other.j.clone()) - (self.i.clone()*other.k.clone())
            + (self.j.clone()*other.x.clone()) + (self.k.clone()*other.i.clone());
        let k = (self.x.clone()*other.k.clone()) + (self.i.clone()*other.j.clone())
            - (self.j.clone()*other.i.clone()) + (self.k.clone()*other.x.clone());
        Quaternion { x: x as T, i: i as T, j: j as T, k: k as T }
    }
}

impl<T: num::Num + clone::Clone> num::One for Quaternion<T> {
    #[inline]
    fn one() -> Self {
        Quaternion { x: T::one(), i: T::zero(), j: T::zero(), k: T::zero() }
    }
}

impl<T: num::Num + clone::Clone> ops::Sub for Quaternion<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl<T: num::Num + clone::Clone> ops::Div for Quaternion<T> {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        let n = T::one() / (
            other.x.clone()*other.x.clone() + other.i.clone()*other.i.clone()
            + other.j.clone()*other.j.clone() + other.k.clone()*other.k.clone()
            );
        let x = other.x * n.clone();
        let i = T::zero()-(other.i * n.clone());
        let j = T::zero()-(other.j * n.clone());
        let k = T::zero()-(other.k * n.clone());
        self * Quaternion { x: x as T, i: i as T, j: j as T, k: k as T }
    }
}

impl<T: num::Num + clone::Clone> ops::Neg for Quaternion<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Quaternion { x: T::zero()-self.x, i: T::zero()-self.i, j: T::zero()-self.j, k: T::zero()-self.k }
    }
}

impl<T: fmt::Display> fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{}i,{}j,{}k)", self.x, self.i, self.j, self.k)
    }
}

impl<T: num::Float + clone::Clone> cmp::Eq for Quaternion<T> {}

impl<T: num::Float + clone::Clone> hash::Hash for Quaternion<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.x.integer_decode().hash(state);
        self.i.integer_decode().hash(state);
        self.j.integer_decode().hash(state);
        self.k.integer_decode().hash(state);
    }
}

impl<T: num::Float + clone::Clone> Quaternion<T> {
    fn norm_sqr(&self) -> T {
        ((self.x.clone()*self.x.clone())
        +(self.i.clone()*self.i.clone())
        +(self.j.clone()*self.j.clone())
        +(self.k.clone()*self.k.clone()))
    }

    pub fn norm(&self) -> T {
        self.norm_sqr().sqrt()
    }
}

