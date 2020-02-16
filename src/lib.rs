struct IterRange {
    lower_bound: isize,
    upper_bound: isize,
    increment: isize,
    current: isize, 
}

impl IterRange {
    fn new(lower_bound: isize, upper_bound: isize, increment: isize) -> IterRange {
        IterRange {
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            increment: increment,
            current: lower_bound,
        }
    }

    fn set_lower(&mut self, lower_bound: isize) {
        
    }

    fn invert() -> IterRange {
        IterRange {
            lower_bound: upper_bound,
            upper_bound: lower_bound,
            increment: -increment,
            current: upper_bound,
        }
    }
}

impl Iterator for IterRange {
    fn next(&mut self) -> Option<isize> {
        self.current += increment;
        if self.current == self.upper_bound {
            return None;
        }
        return Some(self.current);
    }
}