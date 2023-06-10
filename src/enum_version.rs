// define E as in the Haskel example
// data E = One | Zero | ETrue | EFalse | Plus E E | Mult E E | Or E E

// option 1
pub enum Exp {
    ETrue{},
    EFalse {},
    Zero {},
    One {},
    Plus {
        left: Box<Exp>,
        right: Box<Exp>
    },
    Mult{
        left: Box<Exp>,
        right: Box<Exp>
    },
}

fn main() {
    println!("Hello world");

}
