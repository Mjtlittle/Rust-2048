pub struct IterRange {
    lower_bound: isize,
    upper_bound: isize,
    increment: isize,
    start: isize,
    current: isize,
}

impl IterRange {
    pub fn new(lower_bound: isize, upper_bound: isize, increment: isize) -> IterRange {
        IterRange {
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            increment: increment,
            start: lower_bound,
            current: lower_bound,
        }
    }

    pub fn reset(&mut self) {
        self.current = self.start;
    }

    pub fn invert(&self) -> IterRange {
        IterRange {
            lower_bound: self.upper_bound,
            upper_bound: self.lower_bound,
            increment: -self.increment,
            start: self.upper_bound,
            current: self.upper_bound,
        }
    }
}

impl Iterator for IterRange {

    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        
        self.current += self.increment;
        if self.current == self.upper_bound {
            return None;
        }
        return Some(self.current - self.increment);
    }
}