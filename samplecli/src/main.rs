use clap::{App, Arg};
// use clap::Clap;

// #[derive(Clap, Debug)]
// #[clap(
//     name = "My RPN program",
//     version = "1.0.0",
//     author = "yuya-okada527",
//     about = "Super awesome sample RPN calculator"
// )]
// struct Opts {
//     // Sets the level of verbosity
//     #[clap(short, long)]
//     verbode: bool,

//     // Formulas written in RPN
//     #[clap(name = "FILE")]
//     formula_file: Option<String>,
// }

fn main() {

    // program
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("yuya-okada527")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .about("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);

    // derive
    // let opts = Opts::parse();
    // match opts.formula_file {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified."),
    // }
    // println!("Is Verbosity specified?: {}", opts.verbose);
}
