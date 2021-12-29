use std::{rc::Rc, sync::Arc};

fn main() {
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Box::new(30));

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d)
}
