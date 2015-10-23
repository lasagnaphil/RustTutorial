fn vec_min(v: &Vec<i32>) -> Option<i32> {
    use std::cmp;

    let mut min = None;

    for e in v.iter() {
        min = Some(match min {
            None => *e,
            Some(n) => cmp::min(n, *e)
        });
    }
    min
}

fn vec_inc(v: &mut Vec<i32>) {
    for e in v.iter_mut() {
        *e += 1;
    }
}

fn shared_borrow_demo() {
    let v = vec![5,4,3,2,1];
    let first = &v[0];
    vec_min(&v);
    vec_min(&v);
    println!("The first element is {}", *first);
}

fn mutable_borrow_demo() {
    let mut v = vec![5,4,3,2,1];
    vec_inc(&mut v);
    vec_inc(&mut v);
}

fn main() {
    shared_borrow_demo();
    mutable_borrow_demo();
}
