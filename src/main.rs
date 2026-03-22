use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kaiseki", about = "APK analysis workbench")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze an APK file
    Analyze {
        /// Path to the APK file
        apk: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { apk } => {
            println!("kaiseki: analyzing {apk}");
        }
    }
}
