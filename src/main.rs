use clap::{AppSettings, Parser, Subcommand};

mod consts;
mod scales;

use scales::*;

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
    Scale { name: Option<String> },
    Chord { name: Option<String> },
}


fn main() {
    println!("Scales and Chords little helper");
    let cli = Cli::parse();
    match cli.command {
        Commands::Scale { name } => {
            let scale_name: String = name.unwrap_or("Cmaj".to_string());
            let scale: Scale = Scale::parse(&scale_name).unwrap();
            let notes: Vec<&str> = scale.to_notes();

            println!("Scale name: {:?}", scale_name);
            println!("Notes: {:?}", notes);
        }
        Commands::Chord { name } => {
            let chord_name: String = name.unwrap_or("Cmaj".to_string());
            let chord: Chord = Chord::parse(&chord_name).unwrap();
            let notes: Vec<&str> = chord.to_notes();

            println!("Chord name: {:?}", chord_name);
            println!("Chord: {:?}", chord);
            println!("Notes: {:?}", notes);
        }
    }
}

