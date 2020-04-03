// iterrange is an iterator from start to end (both inclusive)

pub struct IteratorRange {
    start: isize,
    end: isize,
    step: isize,

    current: Option<isize>,
}

impl IteratorRange {

    pub fn new(start: isize, end: isize) -> Self {
        IteratorRange {
            start: start,
            end: end,
            step: 1,
            
            current: None,
        }
    }

    pub fn reset(&mut self) {
        self.current = None;
    }

    pub fn clone(&mut self) -> Self {
        IteratorRange {
            start: self.start,
            end: self.end,
            step: self.step,

            current: None,
        }
    }

    pub fn shift(&mut self, delta: isize) -> &Self {
        self.start += delta;
        self.end += delta;
        self.reset();
        return self;
    }

    pub fn reverse(&mut self) -> &Self {
        
        //swap the start and the end
        let temp = self.start;
        self.start = self.end;
        self.end = temp;

        // inverse the step size
        self.step *= -1;

        // and reset
        self.reset();

        return self;
    }
}

impl Iterator for IteratorRange {

    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        
        // if the iterator has not been initialized
        if self.current.is_none() {
            self.current = Some(self.start);
            return self.current;
        }

        // increment the iterator
        self.current = Some(self.current.unwrap() + self.step);
        
        // check if the end if met
        if self.step < 0 {
            if self.current.unwrap() < self.end {
                return None;
            }
        } else {
            if self.current.unwrap() > self.end {
                return None;
            }
        }
        return self.current;
    }
}