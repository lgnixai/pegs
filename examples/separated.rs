use nom::{
    IResult,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
};

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse::<i32>)(input)
}

fn parse_comma_separated_numbers(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(','), parse_number)(input)
}

fn main() {
    let input = "1,2,3,4,5";
    let result = parse_comma_separated_numbers(input);
    match result {
        Ok((remaining, numbers)) => {
            println!("Parsed numbers: {:?}", numbers);
            println!("Remaining input: {}", remaining);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
