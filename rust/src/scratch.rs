use std::{cell::RefCell, ops::Deref, rc::Rc};

use List::{Cons, Nil};

pub fn run() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(2, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let mut x1 = 2;
    let x2 = &mut x1;
    *x2 = 23;
    let x3 = &mut x1;
    *x3 = 2;
    {
        let x1 = RefCell::new(2);
        let mut x2 = x1.borrow_mut();
        *x2 = 23;

        let mut x3 = x1.borrow_mut();
        *x3 = 2;
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn to_vec(&self) -> Vec<i32> {
        let mut value: &List = self;
        let mut result: Vec<i32> = Vec::new();
        loop {
            match value {
                Cons(i, next) => {
                    result.push(*i);
                    value = next;
                }
                Nil => {
                    return result;
                }
            }
        }
    }
}
