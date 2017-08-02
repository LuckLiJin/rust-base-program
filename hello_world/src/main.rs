use std::option::Option;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug,  Clone, Copy, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

pub fn test_fn() -> Option<Vec<PostfixToken>>
{
    None
}
fn main() {
    let mut a = PostfixToken::Operator(Operator::Add);


    let b = Operator::Add;
    let c = Operator::Add;

    if b == Operator::Add && c == Operator::Add
    {
        println!("yes");
    }

    println!("Hello, world!");

}
