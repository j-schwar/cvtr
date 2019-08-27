extern crate structopt;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "CLI numeric base converter", author = "")]
struct Opt {
    /// Convert number to hexadecimal
    #[structopt(short = "x", long = "hex")]
    display_hex: bool,

    /// Convert number to decimal
    #[structopt(short = "d", long = "dec")]
    display_dec: bool,

    /// Convert number to octal
    #[structopt(short = "o", long = "oct")]
    display_oct: bool,

    /// Convert number to binary
    #[structopt(short = "b", long = "bin")]
    display_bin: bool,

    /// Radix of input number
    #[structopt(short = "r", long = "radix")]
    radix: Option<u32>,

    /// Number to convert
    number: String,
}

impl Opt {
    /// Returns `true` if this `Opt` has at least one conversion flag set.
    fn has_conversion_flag(&self) -> bool {
        self.display_bin || self.display_dec || self.display_hex || self.display_oct
    }
}

/// Determines the radix of a numeric string retuning the radix along with a
/// slice of the string without the radix marker.
/// 
/// # Examples
/// 
/// ```
/// assert_eq!(extract_radix("0x100a"), (16, "100a"));
/// ```
/// 
/// # Panics
/// 
/// This function panics if `number` is an empty string.
fn extract_radix(number: &str) -> (u32, &str) {
    assert!(number.len() != 0);
    if number.len() > 2 && &number[0..2] == "0x" {
        return (16, &number[2..]);
    }
    if number.len() > 1 && &number[0..1] == "0" {
        return (8, &number[1..]);
    }
    (10, &number)
}

/// Displays `n` using the various bases set by flags in `opt`.
fn display(n: u32, opt: &Opt) {
    if opt.display_bin {
        println!("binary:  {:b}", n);
    }
    if opt.display_dec {
        println!("decimal: {}", n);
    }
    if opt.display_hex {
        println!("hex:     {:x}", n);
    }
    if opt.display_oct {
        println!("octal:   {:o}", n);
    }
}

/// Runs the main program body so that we can wrap the result in a call to
/// `std::process::exit` for various exit codes.
fn run() -> Result<(), ()> {
    let opt = Opt::from_args();
    if !opt.has_conversion_flag() {
        eprintln!("base: at least one conversion flag must be supplied");
        return Err(());  
    };
    let (radix, num) = match opt.radix {
        Some(r) => (r, opt.number.as_str()),
        None => extract_radix(&opt.number)
    };
    let n = u32::from_str_radix(num, radix);
    if n.is_err() {
        eprintln!("base: failed to parse '{}' with radix = {}", num, radix);
        return Err(());
    };
    display(n.unwrap(), &opt);
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(()) => 1
    })
}
