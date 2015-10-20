
enum NumberOrNothing {
    Number(i32),
    Nothing
}

use self::NumberOrNothing::{Number,Nothing};

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b {a} else {b}
    }

    let mut min = Nothing;
    for el in vec {
        min = Number(match min {
            Nothing => el,
            Number(n) => min_i32(n, el)
        });
    }
    min
}

fn vec_sum(vec: Vec<i32>) -> NumberOrNothing { 
    let mut sum = Nothing;
    for el in vec {
        sum = Number(match sum {
            Nothing => el,
            Number(n) => n + el
        });
    }
    sum
}

fn vec_print(vec: Vec<i32>) {
    let mut print = Nothing;
    for el in vec {
        print!("{}", el);
    }
}

impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        };
    }
}

fn read_vec() -> Vec<i32> {
    vec![1,2,3,4,5,6]
}

pub fn main() {
    let vec = read_vec();
    vec_print(vec);
}
