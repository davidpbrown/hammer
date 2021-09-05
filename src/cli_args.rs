//! Module cli_args handles options and returns other arguments.
//!
//! It will handle commandline options for the main.rs and pass back any arguements that were not set as options.
//!
//! By default acknowledges [-h --help] and [-V --version]
//!
//! # Examples
//! ```
//! extern crate getopts;
//!
//! pub mod cli_args;
//!
//! fn main() {
//! let args: Vec<String> = cli_args::get_args();
//!
//! // Example dump of arguments that were not options
//! println!("{:?}", args);
//!
//! }
//! ```

use getopts::Options;
use std::env;
use std::process;

static PROGRAM: &'static str = "SAFE Network - hammer";
static VERSION: &'static str = "0.0.2";

/// Returns a Vec\<String\> of arguements that were not options.
///
/// Function will exit with error, if the parsing of options fails
/// and also following from certain options, such as help and version.
pub fn get_args() -> Vec<String> {

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("V", "version", "output version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            process::exit(1);
        }
    };

    if matches.opt_present("help"){
	//^ We could as well have used the short name: "h"
       	println!("\nNAME\n\t{}\n\tVersion: {}\n\nSUMMARY\n\tThe simplest front end to the SAFE Network.", PROGRAM, VERSION);
       	let usage = opts.usage("\nUSAGE:\n\t$ hammer\n\n\tOptional: language code ISO639-1\n\t$ hammer en\n\t$ hammer ko");
        println!("{}", usage);
	println!("DESCRIPTION\n\thttps://safenetwork.tech/\n\thttps://safenetforum.org/");
        println!("\nAUTHOR\n\tDavid P Brown\n\nREPORTING BUGS\n\tReport bugs to bugs@davidpbrown.co.uk\n\nCOPYRIGHT\n\tCopyleft!\n");
	process::exit(0);
        }

    if matches.opt_present("version") {
        println!("{}\nVersion: {}", PROGRAM, VERSION);
        process::exit(0);
        }

    if !matches.free.is_empty() { matches.free } else { Vec::<String>::new() }

}
