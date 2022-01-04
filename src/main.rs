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

#[derive(Debug)]
enum Accidentals { Sharp, Flat, Natural }
#[derive(Debug)]
enum ScaleTypes { Maj, Min }

const scale_sharps: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const scale_flats: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

const major_scale_rule: [u8; 7] = [2, 2, 1, 2, 2, 2, 1];  // in semitones
const nat_minor_scale_rule: [u8; 7] = [2, 1, 2, 2, 1, 2, 2];  // in semitones

#[derive(Debug)]
struct Scale {
    key: String,
    accidental: Accidentals,
    scale_type: ScaleTypes,
}

fn parse_scale(scale_name: &str) -> Result<Scale, &str> {
    if scale_name.len() < 4 {
        return Err("Error parsing scale name")
    }
    let accidental = match scale_name.chars().nth(1) {
        Some('#') => Accidentals::Sharp,
        Some('b') => Accidentals::Flat,
        _ => Accidentals::Natural
    };
    let key = match accidental {
        Accidentals::Sharp | Accidentals::Flat => scale_name[..2].to_string(),
        Accidentals::Natural => scale_name[..1].to_string() 
    };
    let scale_type_substr = match accidental {
        Accidentals::Sharp | Accidentals::Flat => scale_name[2..].to_string(),
        Accidentals::Natural => scale_name[1..].to_string()
    };
    let scale_type = match scale_type_substr.as_str() {
        "maj" => ScaleTypes::Maj,
        "min" => ScaleTypes::Min,
        _ => return Err("Error parsing scale name, wrong scale type")
    };
    Ok(Scale { key, accidental, scale_type })
}

fn notes_in_scale(scale_name: &str) -> String {
    "A-B-C-D-E-F-G-A".to_string()
}

fn main() {
    println!("Scales and Chords little helper");
    let cli = Cli::parse();
    match cli.command {
        Commands::Scale { name } => {
            let scale_name: String = name.unwrap_or("Cmaj".to_string());
            println!("Scale name: {:?}", scale_name);
            println!("Scale: {:#?}", parse_scale(&scale_name).unwrap());
            // println!("Notes in scale {:?} : {:?}", scale_name, notes_in_scale(&scale_name));
        }
        Commands::Chords => {
            println!("Hello");
        }
    } 
}
