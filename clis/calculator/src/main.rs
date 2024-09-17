use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, author, version, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add two numbers
    Add { a: i32, b: i32 },
    /// Subtract two numbers
    Subtract { a: i32, b: i32 },
    /// Multiply two numbers
    Multiply { a: i32, b: i32 },
    /// Divide two numbers
    Divide { a: i32, b: i32 },
    /// Perform a % b
    Mod { a: i32, b: i32 },
    /// Raise a to power of b
    Power { a: i32, b: i32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => match cmd {
            Commands::Add { a, b } => println!("{}", calculator::add(a, b)),
            Commands::Subtract { a, b } => println!("{}", calculator::subtract(a, b)),
            Commands::Multiply { a, b } => println!("{}", calculator::multiply(a, b)),
            Commands::Divide { a, b } => println!("{}", calculator::divide(a, b)),
            Commands::Mod { a, b } => println!("{}", calculator::reminder(a, b)),
            Commands::Power { a, b } => println!("{}", calculator::power(a, b)),
        },
        None => {}
    };
}
