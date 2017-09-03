
extern crate rayon;
extern crate num;

use quaternion::Quaternion;
use self::rayon::prelude::*;
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
    fractal: Box<Fn(Quaternion<T>) -> Option<f32>>
}

unsafe impl<T: Send + Sync + 'static> Sync for Renderer<T> {}

impl<T> Renderer<T>
    where T: num::Float + Send + Sync + 'static,
          for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
          for<'a> &'a T: std::ops::Sub<&'a T, Output = T>
{
    pub fn new(func: Box<Fn(Quaternion<T>) -> Option<f32>>) -> Renderer<T> {
        Renderer { fractal: func }
    }

    fn render_pixel(&self, x: T, y: T) -> char {
        let r = (*self.fractal)(Quaternion { x: x, i: y, j: T::zero(), k: T::zero() });
        match r {
            Some(n) if n < 0.1 => '0',
            Some(n) if n < 0.2 => '1',
            Some(n) if n < 0.3 => '2',
            Some(n) if n < 0.4 => '3',
            Some(n) if n < 0.5 => '4',
            Some(n) if n < 0.6 => '5',
            Some(n) if n < 0.7 => '6',
            Some(n) if n < 0.8 => '7',
            Some(n) if n < 0.9 => '8',
            Some(n) if n < 1.0 => '9',
            _ => ' '
        }
    }

    fn render_row(&self, xs: &[T], y: T) -> String {
        let mut row = String::new();
        let iter = xs.par_iter();
        for pixel in iter.map(|&x| self.render_pixel(x, y)).collect::<Vec<char>>() {
            row.push(pixel);
        };
        row
    }

    pub fn render(&self, xs: StepRange<T>, ys: std::iter::Rev<StepRange<T>>) -> Vec<String> {
        let mut screen = Vec::new();
        let vecx = xs.collect::<Vec<T>>();
        let vecy = ys.collect::<Vec<T>>();
        let iter = vecy.par_iter();
        for row in iter.map(|&y| self.render_row(vecx.as_slice(), y)).collect::<Vec<String>>() {
            screen.push(row);
        };
        screen
    }
}

