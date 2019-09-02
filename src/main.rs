extern crate structopt;

use cvtr::radix;
use structopt::StructOpt;

#[derive(Debug, PartialEq)]
enum ArgumentError {
    MultipleInputRadixFlags,
}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ArgumentError::MultipleInputRadixFlags => write!(f, "multiple input radices defined"),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "CLI numeric base converter", author = "")]
struct Opt {
    /// Convert number to hexadecimal
    #[structopt(short = "x")]
    display_hex: bool,

    /// Convert number from hexadecimal
    #[structopt(short = "X")]
    from_hex: bool,

    /// Convert number to decimal
    #[structopt(short = "d")]
    display_dec: bool,

    /// Convert number from decimal
    #[structopt(short = "D")]
    from_dec: bool,

    /// Convert number to octal
    #[structopt(short = "o")]
    display_oct: bool,

    /// Convert number from octal
    #[structopt(short = "O")]
    from_oct: bool,

    /// Convert number to binary
    #[structopt(short = "b")]
    display_bin: bool,

    /// Convert number from binary
    #[structopt(short = "B")]
    from_bin: bool,

    /// Number to convert
    number: String,
}

impl Opt {
    /// Returns the output radices to convert to.
    fn get_output_radices(&self) -> Vec<u32> {
        if !self.display_bin && !self.display_oct && !self.display_dec && !self.display_hex {
            return vec![2, 8, 10, 16];
        }
        let mut v = Vec::new();
        let mut cond_push = |flag, radix| {
            if flag {
                v.push(radix);
            };
        };
        cond_push(self.display_bin, 2);
        cond_push(self.display_oct, 8);
        cond_push(self.display_dec, 10);
        cond_push(self.display_hex, 16);
        v
    }

    /// Returns the radix of the input number.
    fn get_input_radix(&self) -> Result<Option<u32>, ArgumentError> {
        let mut input_radix = None;
        let check = |flag| {
            if input_radix != None {
                Err(ArgumentError::MultipleInputRadixFlags)
            } else {
                Ok(flag)
            }
        };
        input_radix = if check(self.from_bin)? {
            Some(2)
        } else if check(self.from_oct)? {
            Some(8)
        } else if check(self.from_dec)? {
            Some(10)
        } else if check(self.from_hex)? {
            Some(16)
        } else {
            None
        };
        Ok(input_radix)
    }
}

/// Runs the main program body so that we can wrap the result in a call to
/// `std::process::exit` for various exit codes.
fn run() -> Result<(), ()> {
    let opt = Opt::from_args();
    let input_radix = opt.get_input_radix().map_err(|e| println!("{}", e))?;
    let output_radices = opt.get_output_radices();
    let (prefix, number) = radix::strip_prefix(&opt.number);
    let input_radix = input_radix.unwrap_or_else(|| radix::detect(prefix).unwrap());
    for out_radix in output_radices {
        let converted =
            radix::convert(number, input_radix, out_radix).map_err(|e| println!("{}", e))?;
        println!("{:<10} {}", radix::as_text(out_radix) + ":", converted);
    }
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(()) => 1,
    })
}
