
#[macro_use] extern crate cached;
#[macro_use] extern crate lazy_static;
extern crate ncurses;
extern crate num;

mod quaternion;
mod terminal;
mod renderer;

use quaternion::Quaternion;
use self::num::Zero;

struct Iterated<T> {
    func: Box<Fn(&Quaternion<T>) -> Quaternion<T>>,
    z: Quaternion<T>
}

impl<T: num::Float + std::clone::Clone + 'static> Iterator for Iterated<T> {
    type Item = Quaternion<T>;

    #[inline]
    fn next(&mut self) -> Option<Quaternion<T>> {
        let next = self.z.clone();
        self.z = (*self.func)(&next);
        Some(next)
    }
}

fn limsup<T>(f: Box<Fn(&Quaternion<T>) -> Quaternion<T>>, maxiter: usize, ceil: T) -> Option<usize>
    where
        T: num::Float + std::clone::Clone + 'static
{
    let iter = Iterated { func: f, z: Quaternion::<T>::zero() };
    iter.take(maxiter).position(|x| x.norm() >= ceil)
}

cached!{ MANDELBROT >>
fn mandelbrot(z: Quaternion<f64>) -> bool = {
    limsup(Box::new(move |x| (x.clone()*x.clone())+z.clone()), 1000, 2f64).is_none()
}}

fn draw(renderer: &renderer::Renderer<f64>, xpos: f64, ypos: f64, width: f64, height: f64) {
    let (cols,rows) = terminal::get_winsize().unwrap();
    let resx = width / (cols - 2) as f64;
    let resy = height / (rows - 2) as f64;
    let xs = renderer::StepRange::new(xpos, xpos+width, resx);
    let ys = renderer::StepRange::new(ypos-height, ypos, resy).rev();
    let screen = renderer.render(xs, ys);
    let mut lines = screen.iter();
    if let Some(row) = lines.next() {
        ncurses::printw(format!("{}", row).as_ref());
        for row in lines {
            ncurses::printw(format!("\n{}", row).as_ref());
        }
    };
}

fn main() {
    ncurses::initscr();
    ncurses::noecho();
    let mut xpos = -2f64;
    let mut ypos = 1f64;
    let mut width = 4f64;
    let mut height = 2f64;
    ncurses::printw("AsciiBrot");
    let r = renderer::Renderer::new(Box::new(mandelbrot));
    draw(&r, xpos, ypos, width, height);
    ncurses::refresh();
    let mut ch = ncurses::getch();
    loop {
        match std::char::from_u32(ch as u32) {
            Some('q') | Some('Q') | Some('') => break,
            Some('h') => { xpos -= 0.1f64 * width; },
            Some('H') => { xpos -= 0.5f64 * width; },
            Some('j') => { ypos -= 0.1f64 * height; },
            Some('J') => { ypos -= 0.5f64 * height; },
            Some('k') => { ypos += 0.1f64 * height; },
            Some('K') => { ypos += 0.5f64 * height; },
            Some('l') => { xpos += 0.1f64 * width; },
            Some('L') => { xpos += 0.5f64 * width; },
            Some(' ') => {
                width /= 2f64;
                height /= 2f64;
                xpos += 0.5f64 * width;
                ypos -= 0.5f64 * height;
            },
            Some('') => {
                xpos -= 0.5f64 * width;
                ypos += 0.5f64 * height;
                width *= 2f64;
                height *= 2f64;
            },
            None => {
                match ch {
                    ncurses::KEY_EXIT => break,
                    _ => ()
                }
            },
            _ => ()
        }
        ncurses::clear();
        draw(&r, xpos, ypos, width, height);
        ncurses::refresh();
        ch = ncurses::getch();
    };
    ncurses::endwin();
}

