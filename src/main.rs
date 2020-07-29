mod calculator;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "tjm", about = "'That's Just Math!' A CLI calculator for all your mathematical needs")]
struct TjmInput {
    #[structopt(parse(from_str))]
    expression: String
}

fn main() {
    let input = TjmInput::from_args();
    println!("{}", calculator::calculate(&input.expression));
}
