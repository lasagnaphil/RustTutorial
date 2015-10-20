pub enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}

pub use self::SomethingOrNothing::*;

pub type NumberOrNothing = SomethingOrNothing<i32>;
pub type FloatOrNothing = SomethingOrNothing<f32>;

impl<T> SomethingOrNothing<T> {
    pub fn new(o: Option<T>) -> Self {
        match o { None => Nothing, Some(t) => Something(t) }
    }
    pub fn to_option(self) -> Option<T> {
        match self { Nothing => None, Something(t) => Some(t) }
    }
}

impl NumberOrNothing {
    pub fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

impl FloatOrNothing {
    pub fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Something(n) => println!("The number is: {}", n),
        };
    }
}

pub trait Minimum : Copy {
    fn min(self, b: Self) -> Self;
}

impl Minimum for i32 {
    fn min(self, b: Self) -> Self {
        if self < b {self} else {b}
    }
}

impl Minimum for f32 {
    fn min(self, b: Self) -> Self {
        if self < b {self} else {b}
    }
}

pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    for e in v {
        min = Something(match min {
            Nothing => e,
            Something(n) => {
                e.min(n)
            }
        });
    }
    min
}


