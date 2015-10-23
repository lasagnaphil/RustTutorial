
#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>,
}

impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    pub fn from_vec(mut v: Vec<u64>) -> Self {
        while v.len() > 0 && v[v.len()-1] == 0 {
            v.pop();
        }
        BigInt { data: v }
    }

    pub fn num_digits(&self) -> u64 {
        let mut count: u64 = 0;
        for e in self.data.iter() {
            if *e == 0 { return count; } else { count += 1; }
        }
        count
    }

    pub fn num_nonzero_digits(&self) -> u64 {
        let mut count: u64 = 0;
        for e in self.data.iter() {
            if *e == 0 { count += 1; }
        }
        count
    }
    
    pub fn num_smallest_digit(&self) -> u64 {
        let mut smallest: u64 = self.data[0];
        for e in self.data.iter() {
            if *e < smallest { smallest = *e; }
        }
        smallest
    }

    pub fn num_biggest_digit(&self) -> u64 {
        let mut biggest: u64 = self.data[0];
        for e in self.data.iter() {
            if *e > biggest { biggest = *e; }
        }
        biggest
    }

    fn min_try1(self, other: Self) -> Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            self
        }
    }

}

pub trait Minimum {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

impl Minimum for BigInt {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        if self.data.len() <= other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other 
        } else {
            let mut idx = self.data.len();
            while idx > 0 {
                idx = idx-1;
                if self.data[idx] < other.data[idx] {
                    return self;
                } else if self.data[idx] > other.data[idx] {
                    return other;
                }
            }
            return self;
        }
    }
}

impl PartialEq for BigInt {
    #[inline]
    fn eq(&self, other: &BigInt) -> bool {
        debug_assert!(self.test_invariant() && other.test_invariant());
        self.data == other.data
    }
}

fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
    let mut min = None;
    for e in v {
        min = Some(match min {
            None => e,
            Some(n) => e.min(n)
        });
    }
    min
}

fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v);
}

enum Variant {
    Number(i32),
    Text(String),
}

fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    *ptr = 1337;
}


use std::fmt;

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

fn main() {
    clone_demo();
}

#[cfg(test)]
mod tests {
    fn test_vec_min() {
        let b1 = BigInt::new(1);
        let b2 = BigInt::new(42);
        let b3 = BigInt::new(0);
        let b4 = BigInt::from_vec(vec![0, 1]);
        let b5 = BigInt::from_vec(vec![0, 1, 0, 0, 0]);

        let v1 = vec![b2.clone(), b1.clone(), b4.clone()];
        let v2 = vec![b2.clone(), b4.clone()];

        assert_eq!(vec_min(&v1), Some(&b1));
        assert_eq!(vec_min(&v2), Some(&b2));
        assert_eq!(vec_min::<BigInt>(&vec![]),None);
        assert_eq!(Some(&b4), Some(&b5));
    }

    #[test]
    fn test_min() {
        let b1 = BigInt::new(1);
        let b2 = BigInt::new(42);
        let b3 = BigInt::from_vec(vec![0,1]);

        assert!(*b1.min(&b2) == b1);
        assert!(*b3.min(&b2) == b2);
    }
}
