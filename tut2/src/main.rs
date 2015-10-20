
pub enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}

pub use self::SomethingOrNothing::*;

type NumberOrNothing = SomethingOrNothing<i32>;
type FloatOrNothing = SomethingOrNothing<f32>;

impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o { None => Nothing, Some(t) => Something(t) }
    }
    fn to_option(self) -> Option<T> {
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

fn read_vec() -> Vec<i32> {
    vec![16,27,34,2,37,13]
}

fn read_vec_float() -> Vec<f32> {
    vec![1.0f32,5.2f32,0.3f32,6.3f32,2.4f32]
}

pub fn main() {
    let vec = read_vec_float();
    let min = vec_min(vec);
    min.print()
}
