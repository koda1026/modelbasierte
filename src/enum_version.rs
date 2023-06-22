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
        Exp::One{} => return Maybe::Just{a: Either::Left{int: 1}},
        Exp::Zero{} => return Maybe::Just{a: Either::Left{int: 0}},
        Exp::Plus{left, right} => {
            let l = eval(left);
            let r = eval(right);
            match (l,r){
                (Maybe::Just{a: Either::Left{int: i1}}, 
                Maybe::Just{a: Either::Left{int: i2}}) 
                => return Maybe::Just{a: Either::Left{int: (i1 + i2)}},

                (Maybe::Nothing{}, _) => todo!(),
                (Maybe::Just{a: Either::Right{boolean: b1}}, _) => todo!(),
                (_,_) => todo!(),
            }
        }

        _ => return Maybe::Nothing{},

    }
}


fn main() {
    let e = Exp::One{};

}
