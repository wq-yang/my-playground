use std::{thread::sleep, time::Duration};
// ANSI Escape Seq
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Bounded {
    len: usize,
    delim: (char, char),
}
struct Unbounded;
trait BoundState {}
impl BoundState for Bounded {}
impl BoundState for Unbounded {}

// Bound is either Bounded or Unbounded
struct Progress<Iter, Bound: BoundState> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

// unbounded by default
impl<Iter> Progress<Iter, Unbounded>
where
    Iter: Iterator,
{
    fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}
impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    fn with_bound(self) -> Progress<Iter, Bounded> {
        let iter = self.iter;
        let iter_len = iter.len();
        Progress {
            iter,
            i: 0,
            bound: Bounded {
                len: iter_len,
                delim: ('[', ']'),
            },
        }
    }
}
// trait ProgressDisplay<Iter, Bound: BoundState> {
//     fn display(&self, progress: &Progress<Iter, Bound>);
// }

// impl<Iter> ProgressDisplay<Iter, Unbounded> for Unbounded {
//     fn display(&self, progress: &Progress<Iter, Unbounded>) {
//         println!("{}{}", CLEAR, "*".repeat(progress.i),);
//     }
// }
// impl<Iter> ProgressDisplay<Iter, Bounded> for Bounded {
//     fn display(&self, progress: &Progress<Iter, Bounded>) {
//         println!(
//             "{}{}{}{}{}",
//             CLEAR,
//             self.delim.0,
//             "*".repeat(progress.i),
//             " ".repeat(self.len - progress.i),
//             self.delim.1
//         );
//     }
// }
impl<Iter> Progress<Iter, Bounded>
where
    Iter: ExactSizeIterator,
{
    fn with_delim(mut self, delim: (char, char)) -> Self {
        self.bound.delim = delim;
        self
    }
}
impl<Iter> Iterator for Progress<Iter, Bounded>
where
    Iter: ExactSizeIterator,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!(
            "{}{}{}{}{}",
            CLEAR,
            self.bound.delim.0,
            "*".repeat(self.i),
            " ".repeat(self.bound.len - self.i),
            self.bound.delim.1
        );
        self.i += 1;
        self.iter.next()
    }
}

impl<Iter> Iterator for Progress<Iter, Unbounded>
where
    Iter: Iterator,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i),);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIterExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}
impl<Iter> ProgressIterExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Iter, Unbounded> {
        Progress::new(self)
    }
}

fn calculation<T>(_x: T) {
    sleep(Duration::from_secs(1));
}

fn main() {
    for x in (1..4).progress().with_bound().with_delim(('<', '>')) {
        calculation(x);
    }
    // for x in (1..).progress().with_bound() { // err in compile time
    for x in (1..).progress() {
        calculation(x);
    }
}
