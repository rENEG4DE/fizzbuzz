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

    for i in (start..end + 1).step_by(step) {
        print!("{}: ", i);
        out_fizz_buzz(fizzbuzz(i));
    }
}

fn out_fizz_buzz(fb: Fb) {
    match fb {
        Fb::Num(val) => println!("{}", val),
        _ => println!("{:?}", fb)
    }
}

fn fizzbuzz(nr: u32) -> Fb {
    let is_fizz = |argument: u32| argument % 3 == 0;
    let is_buzz = |argument: u32| argument % 5 == 0;
    let is_fizz_buzz = |argument: u32| is_fizz(argument) && is_buzz(argument);

    match nr {
        0 => Fb::Num(nr),
        x if is_fizz_buzz(x) => Fb::Fizzbuzz,
        x if is_fizz(x) => Fb::Fizz,
        x if is_buzz(x) => Fb::Buzz,
        _ => Fb::Num(nr)
    }
}

fn handle_arguments(args: Vec<String>) -> (u32, u32, usize) {
    let arg_u32_conv = |argument: &String| argument.parse::<u32>().unwrap();
    let arg_usize_conv = |argument: &String| argument.parse::<usize>().unwrap();

    match args.len() {
        x if x > 4 => panic!("Too many arguments provided - give start, end, step or less arguments"),
        2 => (START, arg_u32_conv(&args[1]), STEP),
        3 => (arg_u32_conv(&args[1]), arg_u32_conv(&args[2]), STEP),
        4 => (arg_u32_conv(&args[1]), arg_u32_conv(&args[2]), arg_usize_conv(&args[3])),
        _ => (START, END, STEP)
    } 
}

#[cfg(test)]
mod tests {
    use super::{fizzbuzz, handle_arguments, Fb, STEP, END, START};

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

    #[test]
    #[should_panic (expected = "Too many arguments provided - give start, end, step or less arguments")]
    fn test_handle_arguments() {
        let arg0: String = std::env::args().nth(0).unwrap();

        assert_eq!(handle_arguments(vec!(arg0.clone())), (START,END,STEP));
        assert_eq!(handle_arguments(vec!(arg0.clone(), String::from("10"))), (START,10,STEP));
        assert_eq!(handle_arguments(vec!(arg0.clone(), String::from("10"), String::from("20"))), (10,20,STEP));
        assert_eq!(handle_arguments(vec!(arg0.clone(), String::from("10"), String::from("20"), String::from("2"))), (10,20,2));
        handle_arguments(vec!(arg0, String::from("some"), String::from("bull"), String::from("shit"), String::from("arguments")));
    }
}
