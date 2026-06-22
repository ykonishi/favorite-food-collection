use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use favorite_food_collection::{generate_favorite_food, GenerateOptions};

#[derive(Parser)]
#[command(name = "favorite-food-collection")]
#[command(about = "Generate a wordcloud from users' favorite food lists")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Collect users/*/list.txt and generate a wordcloud PNG
    FavoriteFood {
        /// Directory containing user subdirectories with list.txt
        #[arg(short, long, default_value = "users")]
        users_dir: PathBuf,

        /// Output PNG path
        #[arg(short, long, default_value = "output/favorite-food.png")]
        output: PathBuf,

        /// Image width (must be a multiple of 64)
        #[arg(long, default_value_t = 1024)]
        width: usize,

        /// Image height (must be a multiple of 64)
        #[arg(long, default_value_t = 1024)]
        height: usize,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::FavoriteFood {
            users_dir,
            output,
            width,
            height,
        } => match generate_favorite_food(&GenerateOptions {
            users_dir: &users_dir,
            output_path: &output,
            width,
            height,
        }) {
            Ok(summary) => {
                println!(
                    "Collected {} list.txt files ({} items, {} unique)",
                    summary.file_count, summary.item_count, summary.unique_count
                );
                println!("Saved wordcloud to {}", summary.output_path);
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("Error: {err:#}");
                ExitCode::from(1)
            }
        },
    }
}
