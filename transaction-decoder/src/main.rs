use clap::Parser;

#[derive(Parser)]
#[command(name="Transaction decoder")]
#[command(version = "1.0")]
#[command(about = "Bitcoin transaction decoder", long_about = None)]
struct Cli {
    #[arg(
        required = true,
        help = "(string, required) Raw transaction hex"
    )]
    transaction_hex: String,
}

fn main() {
    
//    let transaction_hex = "";
    let cli = Cli::parse();
    match transaction_decoder::decode(cli.transaction_hex) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e)
    }
}