use std::env;

use std::error;
use std::fmt;
use std::num;


fn main() {
    // match double_arg(env::args()) {
    //     Ok(n) => println!("{}", n),
    //     Err(err) => println!("error:  {}", err),
    // }
    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err @ CliError::NotEnoughArgs) => println!("error:  {}", err),
        Err(CliError::Parse(..)) => {
            println!("error: invalid number {}",
                     env::args().nth(1).unwrap())
        }
    }
}

#[allow(dead_code)]
/// sample: arg-parse with combinator
fn double_arg1(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("input a number".to_owned())
        .and_then(|x| x.parse::<i32>().map_err(|err| err.to_string()))
        .map(|x| 2 * x)
}


#[allow(dead_code)]
/// sample: arg-parse with combinator and try
fn double_arg2(mut argv: env::Args) -> Result<i32, String> {
    let arg1 = try!(argv.nth(1).ok_or("input a number".to_owned()));
    let x = try!(arg1.parse::<i32>().map_err(|err| err.to_string()));
    Ok(2 * x)
}

#[derive(Debug)]
struct NotEnoughArgError {}

impl fmt::Display for NotEnoughArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "args is missing.")
    }
}

impl error::Error for NotEnoughArgError {
    fn description(&self) -> &str {
        "args is missing."
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug)]
enum CliError {
    NotEnoughArgs,
    Parse(num::ParseIntError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::NotEnoughArgs => write!(f, "args is missing"),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::NotEnoughArgs => "args is missing",
            CliError::Parse(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::NotEnoughArgs => None,
            CliError::Parse(ref err) => Some(err),
        }
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError{
        CliError::Parse(err)
    }
}

#[allow(dead_code)]
/// sample: anti type erasure
fn double_arg4(mut argv: env::Args) -> Result<i32, Box<error::Error>> {
    let arg1 = try!(argv.nth(1).ok_or(NotEnoughArgError{}));
    let x = try!(arg1.parse::<i32>());
    Ok(2 * x)
}


/// sample: full set
fn double_arg(mut argv: env::Args) -> Result<i32, CliError> {
    let arg1 = try!(argv.nth(1).ok_or(CliError::NotEnoughArgs));
    let x = try!(arg1.parse::<i32>());
    Ok(2 * x)
}
