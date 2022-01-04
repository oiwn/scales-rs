use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scale { name: Option<String>}, 
    Chords,
}

const scale_sharps: Vec<&str> = vec!["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const scale_flats: Vec<&str> = vec!["C", "Db", "D", "Eb", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

fn notes_in_scale(scale_name: &str) -> String {
    "A-B-C-D-E-F-G-A".to_string()
}

fn main() {
    println!("Scales and Chords little helper");
    let cli = Cli::parse();
    match &cli.command {
        Commands::Scale { name } => {
            let scale_name = match name {
                None => "Amin".to_string(),
                Some(x) => x.to_string()
            };
            println!("Notes in scale {:?} : {:?}", scale_name, notes_in_scale(&scale_name));
        }
        Commands::Chords => {
            println!("Hello");
        }
    } 
    println!("Hello, world!");
}
