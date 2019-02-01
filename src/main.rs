#[derive(Debug, PartialEq)]
enum Fb {
    Fizz,
    Buzz,
    Fizzbuzz,
    Num(u32),
}

const START: u32 = 0;
const END: u32 = 100;
const STEP: usize = 1;

fn main() {
    let (start, end, step) = handle_arguments(std::env::args().collect());

    for i in (start..end).step_by(step) {
        print!("{}: ", i);
        out_fizz_buzz(fizzbuzz(i));
    }
}

fn handle_arguments(args: Vec<String>) -> (u32, u32, usize) {
    let arg_u32_conv = |argument: &String| argument.parse::<u32>().unwrap();
    let arg_usize_conv = |argument: &String| argument.parse::<usize>().unwrap();

    return match args.len() {
        2 => (START, arg_u32_conv(&args[1]), STEP),
        3 => (arg_u32_conv(&args[1]), arg_u32_conv(&args[2]), STEP),
        4 => (arg_u32_conv(&args[1]), arg_u32_conv(&args[2]), arg_usize_conv(&args[3])),
        _ => (START, END, STEP)
    } 
}

fn out_fizz_buzz(fb: Fb) {
    match fb {
        Fb::Num(val) => println!("{}", val),
        _ => println!("{:?}", fb),
    }
}

fn fizzbuzz(nr: u32) -> Fb {
    let result: Fb;
    if nr == 0 {
        result = Fb::Num(0);
    } else if nr % 3 == 0 && nr % 5 == 0 {
        result = Fb::Fizzbuzz;
    } else if nr % 3 == 0 {
        result = Fb::Fizz;
    } else if nr % 5 == 0 {
        result = Fb::Buzz;
    } else {
        result = Fb::Num(nr);
    }
    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::{fizzbuzz, Fb};

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(0), Fb::Num(0));
        assert_eq!(fizzbuzz(1), Fb::Num(1));
        assert_eq!(fizzbuzz(2), Fb::Num(2));
        assert_eq!(fizzbuzz(3), Fb::Fizz);
        assert_eq!(fizzbuzz(4), Fb::Num(4));
        assert_eq!(fizzbuzz(5), Fb::Buzz);
        assert_eq!(fizzbuzz(15), Fb::Fizzbuzz);
    }
}
