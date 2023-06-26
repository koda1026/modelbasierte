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
        Exp::ETrue{} => return Some(Right(true)),
        Exp::EFalse{} => return Some(Right(false)),
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
        Exp::Mult{left, right} => {
            let l = eval(left);
            let r = eval(right);
            match (l,r){
                (Some(Left(0)), _) => return Some(Left(0)),
                (Some(Left(i1)), Some(Left(i2))) => return Some(Left(i1 * i2)),
                (_, _) => return None,
            }
        }
        _ => return None,

    }
}


fn main() {
    let one = Exp::One{};
    let one2 = Exp::One{};
    let two = Exp::One{};
    let two2 = Exp::One{};
    let result = Exp::Plus{left: Box::new(one), right: Box::new(one2)};
    let result2 = Exp::Plus{left: Box::new(two), right: Box::new(two2)};
    let finale = eval(&Exp::Mult{left: Box::new(result), right: Box::new(result2)});
    if let Option::Some(Either::Left(i1)) = finale{
        println!("{}", i1);
    }
}
