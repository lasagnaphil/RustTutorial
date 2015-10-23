
use std::io::prelude::*;
use std::io;

fn read_vec() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::<i32>::new();
    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D.");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match line.trim().parse::<i32>() {
            Ok(num) => {
                vec.push(num)
            },
            Err(_) => {
                println!("What did I say about numbers?")
            },
        }
    }
    
    vec
}

mod part02;
use part02::{SomethingOrNothing, Something, Nothing, vec_min};

fn read_vec_f32() -> Vec<f32> {
    vec![15.02,37.30,52.26,34.26,32.56]
}

pub fn main()
{
    let vec = read_vec_f32();
    let min = vec_min(vec);
    min.print();
}

trait Print {
    fn print2(self);
}
/*
impl<T: Print> SomethingOrNothing<T> {
    fn print2(self) {
        match self {
            Nothing => println!("<nothing>"),
            Something(n) => print!("{}",n)
        }
    }
}
*/
