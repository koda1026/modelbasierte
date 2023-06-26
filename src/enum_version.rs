use std::option::*;
use Either::*;


pub enum Either<L, R> {
    Left(L),
    Right(R),
}

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

fn eval(e: &Exp) -> Option<Either<i32, bool>> {
    match e{
        Exp::One{} => return Some(Left(1)),
        Exp::Zero{} => return Some(Left(0)),
        Exp::Plus{left, right} => {
            let l = eval(left);
            let r = eval(right);
            match (l,r){
                (Some(Left(i1)), 
                Some(Left(i2))) 
                => return Some(Left(i1 + i2)),
                
                (_,_) => return None,
                
                //Not needed for Plus
                //(Maybe::Nothing{}, _) => todo!(),
                //(Maybe::Just{a: Either::Right{boolean: b1}}, _) => todo!(),
            }
        }

        _ => return None,

    }
}


fn main() {
    let one = Exp::One{};
    let one2 = Exp::One{};
    let result = eval(&Exp::Plus{left: Box::new(one), right: Box::new(one2)});
    if let Option::Some(Either::Left(i1)) = result{
        print!("{}", i1);
    }
}
