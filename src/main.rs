use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Weiwen Chen <17307110121@fudan.edu.cn>")]
struct Opts {
    fuzz_file: String,

    #[clap(short, long, default_value = "cfm_in/")]
    input: String,

    #[clap(short, long, default_value = "cfm_out/")]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Input: {}", opts.input);
    println!("Output: {}", opts.output);
    println!("Fuzzfile: {}", opts.fuzz_file);
}