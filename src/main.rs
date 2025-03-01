use clap::{Parser, Subcommand};
use crate::core::Core;

mod core;
mod config;
mod autocomplete;
mod show;
mod stats;
mod review;

#[derive(Parser)]
#[command(name = "dia")]
#[command(about = "Smart work diary with semantic tracking", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Open the database file
    Db,
    
    /// Log a new diary entry
    Log {
        /// The entry text with semantic tags
        entry: String,
        
        #[arg(short, long)]
        /// Specific date (YYYY-MM-DD)
        date: Option<String>,
    },
    
    /// Show diary entries and entities
    Show {
        #[command(subcommand)]
        target: ShowTarget,
    },
    
    /// Display statistics and insights
    Stats {
        #[arg(short, long)]
        /// Time period to analyze (e.g. "last week", "this month")
        period: Option<String>,
    },
    
    /// Review entries in spaced repetition style
    Review,
}

#[derive(Subcommand)]
enum ShowTarget {
    /// Show entries matching filters
    Entries {
        #[arg(short, long)]
        /// Date or date range (YYYY-MM-DD or YYYY-MM-DD..YYYY-MM-DD)
        date: Option<String>,
        
        #[arg(short, long)]
        /// Search term
        search: Option<String>,
        
        #[arg(short, long)]
        /// Filter by person (@name)
        person: Option<String>,
        
        #[arg(short, long)]
        /// Filter by project (%name)
        project: Option<String>,
        
        #[arg(short, long)]
        /// Filter by tag (#name)
        tag: Option<String>,
    },
    /// List all people
    People,
    /// List all projects
    Projects,
    /// List all tags
    Tags,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut core = Core::init()?;

    match cli.command {
        Commands::Db => {
            let config = config::Config::load()?;
            open::that(config.diary_db_path)?;
            println!("Database opened successfully!");
        }
        Commands::Log { entry, date } => {
            core.add_entry(&entry, date.as_deref())?;
            println!("Entry logged successfully!");
        }
        Commands::Show { target } => {
            show::handle_show_command(target, &core)?;
        }
        Commands::Stats { period } => {
            stats::handle_stats_command(period, &core)?;
        }
        Commands::Review => {
            review::handle_review_command(&core)?;
        }
    }

    Ok(())
}
