use core::fmt;
use std::{
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
    ArithmeticError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
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
        println!("{}, {}", self, other);
        match (self, other) {
            (Object::Str(left), Object::Str(right)) => {
                println!("Got string");
                Object::Str(format!("{}{}", left, right))
            }
            (Object::Num(left), Object::Num(right)) => Object::Num(left + right),
            _ => Object::ArithmeticError,
        }
    }
}
