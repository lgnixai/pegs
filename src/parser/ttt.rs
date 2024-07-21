use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, space0};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, tuple};
use nom::branch::alt;
use nom::combinator::map;
use crate::ast::atom::Atom;
use crate::ast::expression::Expression;

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(digit1, |s: &str| {
            Expression::Atom(Atom::Integer(s.parse().unwrap()))
        }),
        map(preceded(space0, delimited(char('"'), alpha1, char('"'))), |s: &str| {
            Expression::Atom(Atom::String(s.to_string()))
        }),
    ))(input)
}

fn parse_method_call(input: &str) -> IResult<&str, Expression> {
    // Example: ta.change("Bull", 22);
    let (input, (obj_name, _, method_name, _, args, _)) = tuple((
        alpha1,
        tag("."),
        alpha1,
        char('('),
        separated_list0(tag(", "), parse_expression),
        char(')'),
    ))(input)?;

    Ok((input, Expression::MethodCall(obj_name.to_string(), method_name.to_string(), args)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expression::Expression;

    #[test]
    fn parse_method_call_test() {
        let input = "ta.change(\"Bull\", 22)";
        let result = parse_method_call(input);
        println!("{:?}", result);


    }
}
