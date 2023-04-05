use core::fmt;
use std::{
    cmp::Ordering,
    fmt::format,
    ops::{Add, Div, Mul, Sub},
};

// TODO: Seprate floating point with int
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    // TODO: Update arithmatic error
    ArithmeticError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Nil => write!(f, "Nil"),
            Object::Bool(x) => {
                if *x {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Object::ArithmeticError => panic!("Should not be trying to print this"),
        }
    }
}

impl Sub for Object {
    type Output = Object;

    fn sub(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left - right),
            _ => Object::ArithmeticError,
        }
    }
}

impl Div for Object {
    type Output = Object;

    fn div(self, other: Self) -> Object {
        match other {
            Object::Num(other) => {
                if other == 0.0 {
                    return Object::ArithmeticError;
                }
            }
            _ => (),
        }

        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left / right),
            _ => Object::ArithmeticError,
        }
    }
}

impl Mul for Object {
    type Output = Object;

    fn mul(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left * right),
            _ => Object::ArithmeticError,
        }
    }
}

impl Add for Object {
    type Output = Object;

    fn add(self, other: Self) -> Object {
        match (self, other) {
            (Object::Str(left), Object::Num(right)) => {
                Object::Str(format!("{}{}", left, right.to_string()))
            }
            (Object::Num(left), Object::Str(right)) => {
                Object::Str(format!("{}{}", left.to_string(), right))
            }
            (Object::Str(left), Object::Str(right)) => Object::Str(format!("{}{}", left, right)),
            (Object::Num(left), Object::Num(right)) => Object::Num(left + right),
            _ => Object::ArithmeticError,
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => left.partial_cmp(right).unwrap(),
            _ => Ordering::Equal,
        }
    }
}

impl Eq for Object {}
