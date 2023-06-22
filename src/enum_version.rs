// define E as in the Haskel example
// data E = One | Zero | ETrue | EFalse | Plus E E | Mult E E | Or E E

// option 1
pub enum Exp {
    ETrue{},
    EFalse {},
    One{},
    Zero{},
    Plus {
        left: Box<Exp>,
        right: Box<Exp>
    },
    Mult{
        left: Box<Exp>,
        right: Box<Exp>
    },
    Or{
        left: Box<Exp>,
        right: Box<Exp>
    }
}

fn eval(e: &Exp) -> i32 {
    match e{
        Exp::One{} => return 1,
        Exp::Zero{} => return 0,
        Exp::Plus{left, right} => return eval(left) + eval(right),
        Exp::Mult{left, right} => return eval(left) * eval(right),

        _ => return -1,

    }
}


fn main() {
    let e = Exp::One{};
    println!("{}", eval(&e));

}
