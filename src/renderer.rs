
extern crate num;

use quaternion::Quaternion;
use std;

pub struct StepRange<T>
    where T: num::Float + Clone,
          for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
          for<'a> &'a T: std::ops::Sub<&'a T, Output = T>
{
    start: T,
    stop: T,
    step: T,
    orig_start: T,
    orig_stop: T
}

impl<T> StepRange<T>
    where T: num::Float + Clone,
          for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
          for<'a> &'a T: std::ops::Sub<&'a T, Output = T>
{
    pub fn new(start: T, stop: T, step: T) -> StepRange<T> {
        let orig_start = start.clone();
        let orig_stop = stop.clone();
        StepRange { start, stop, step, orig_start, orig_stop }
    }
}

impl<T> Iterator for StepRange<T>
    where T: num::Float + Clone,
          for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
          for<'a> &'a T: std::ops::Sub<&'a T, Output = T>
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.start < self.stop {
            let v = self.start.clone();
            self.start = &v + &self.step;
            Some(v)
        } else {
            self.start = self.orig_start;
            self.stop = self.orig_stop;
            None
        }
    }
}

impl<T> DoubleEndedIterator for StepRange<T>
    where T: num::Float + Clone,
          for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
          for<'a> &'a T: std::ops::Sub<&'a T, Output = T>
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if self.start < self.stop {
            let v = self.stop.clone();
            self.stop = &v - &self.step;
            Some(v)
        } else {
            self.start = self.orig_start;
            self.stop = self.orig_stop;
            None
        }
    }
}

pub struct Renderer<T> {
    fractal: Box<Fn(Quaternion<T>) -> bool>
}

impl<T> Renderer<T>
    where T: num::Float,
          for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
          for<'a> &'a T: std::ops::Sub<&'a T, Output = T>,
          T: std::convert::From<i8>
{
    pub fn new(func: Box<Fn(Quaternion<T>) -> bool>) -> Renderer<T> {
        Renderer { fractal: func }
    }

    fn renderpixel(&self, x: T, y: T) -> char {
        let r = (*self.fractal)(Quaternion { x: x, i: y, j: T::zero(), k: T::zero() });
        match r {
            true => '*',
            false => ' '
        }
    }

    fn renderrow(&self, xs: &mut StepRange<T>, y: T) -> String {
        let mut row = String::new();
        for x in xs {
            row.push(self.renderpixel(x, y));
        };
        row
    }

    pub fn render(&self, mut xs: StepRange<T>, ys: std::iter::Rev<StepRange<T>>) -> Vec<String> {
        let mut screen = Vec::new();
        for y in ys {
            screen.push(self.renderrow(&mut xs, y));
        };
        screen
    }
}

