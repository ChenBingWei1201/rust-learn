use std::iter::Iterator;

struct  Counter {
    count: u32,
}

impl Counter {
    fn new(c: u32) -> Counter {
        Counter { count: c }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        if self.count == 0 {
            None
        } else {
            let x = self.count;
            self.count -= 1;
            Some(x)
        }
    }
}

fn main() {
    let mut counter = Counter::new(10);
    println!("{}", counter.next().unwrap());
    println!("{}", counter.next().unwrap());
}
