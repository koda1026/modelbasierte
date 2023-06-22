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
pub enum Either{
    Left{
        int: i32,
    },
    Right{
        boolean: bool,
    }
}
pub enum Maybe {
    Just{
        a: Either,
    },
    Nothing{}
}

fn eval(e: &Exp) -> Maybe {
    match e{
        Exp::One{} => return Maybe::Just::Left{int: 1},
        Exp::Zero{} => return Maybe::Just::Left{int: 0},
        Exp::Plus{left, right} => 
            Maybe l = eval(left);
            Maybe r = eval(right);
            match l,r{
                Maybe::Just{a: Either::Left}, 
                Maybe::Just{a: Either::Left} => Maybe::Just{a: Either::Left{l+r}};
            }
        Exp::Mult{left, right} => return eval(left) * eval(right),

        _ => return -1,

    }
}


fn main() {
    let e = Exp::One{};

}
