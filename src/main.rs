use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "calc")]
#[command(about = "A calculater with multiple operations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add two numbers
    Sum {
        /// First number
        num1: f64,
        /// Second number
        num2: f64,
    },

    /// Substract two numbers
    Sub {
        /// First number
        num1: f64,
        /// Second number
        num2: f64,
    },

    /// Multiply two numbers
    Mul {
        /// First number
        num1: f64,
        /// Second number
        num2: f64,
    },

    /// Divide two numbers
    Div {
        /// First number
        num1: f64,
        /// Second number
        num2: f64,
    },
    
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Sum { num1, num2 } => println!("Result: {}\n", num1 + num2),
        Commands::Sub { num1, num2 } => println!("Result: {}\n", num1 - num2),
        Commands::Mul { num1, num2 } => println!("Result: {}\n", num1 * num2),
        Commands::Div { num1, num2 } => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero is not allowed");
                std::process::exit(1);
            }
            println!("Result: {}\n", num1 + num2);
        }
    }
}
