// define E as in the Haskel example
// data E = One | Zero | ETrue | EFalse | Plus E E | Mult E E | Or E E

trait Exp{
    fn eval(&self) -> Box<dyn Exp> // Maybe Either Int Bool
}

struct ETrue;
struct EFalse;
struct Zero;
struct One;

struct Plus {
    left: Box<dyn Exp>,
    right: Box<dyn Exp>
}
struct Mult {
    left: Box<dyn Exp>,
    right: Box<dyn Exp>
}
struct Or {
    left: Box<dyn Exp>,
    right: Box<dyn Exp>
}



impl Exp for ETrue {
    fn eval(&self) -> bool {
        return true;
    }
}

impl Exp for EFalse {
    fn eval(&self) -> bool {
        return false;
    }
}

impl Exp for Zero {
    fn eval(&self) -> i32 {
        return 0;
    }
}

impl Exp for One {
    fn eval(&self) -> i32 {
        return 1;
    }
}

impl Exp for Plus {
    fn eval(&self) -> i32 {
        r = self.right.eval();
        // left is no number
        l = self.left.eval();
        if (l /*not a number*/) {
            return /*Fail*/;
        }
        // right is no number
        r = self.right.eval();
        if (r /*not a number*/) {
            return /*Fail*/;
        }
        // both are numbers

        return l + r;
    }
}

impl Exp for Mult {
    fn eval(&self) -> i32 {
        r = self.right.eval();
        // left is no number
        l = self.left.eval();
        if (l /*not a number*/) {
            return /*Fail*/;
        }
        // right is no number
        r = self.right.eval();
        if (r /*not a number*/) {
            return /*Fail*/;
        }
        // both are numbers

        return l * r;
    }
}

impl Exp for Or {
    fn eval(&self) -> i32 {
        r = self.right.eval();
        // left is no bool
        l = self.left.eval();
        if (l /*not a bool*/) {
            return /*Fail*/;
        }
        // right is no bool
        r = self.right.eval();
        if (r /*not a bool*/) {
            return /*Fail*/;
        }
        // both are bools

        return l || r;
    }
}

fn main() {
    println!("Hello world");

}
