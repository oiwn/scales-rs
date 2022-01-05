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

#[derive(Debug, PartialEq)]
enum Accidentals { Sharp, Flat, Natural }
#[derive(Debug, PartialEq)]
enum ScaleTypes { Maj, Min }

const SCALE_SHARPS: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const SCALE_FLATS: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

const MAJOR_SCALE_RULE: [u8; 7] = [2, 2, 1, 2, 2, 2, 1];  // in semitones
const NAT_MIN_SCALE_RULE: [u8; 7] = [2, 1, 2, 2, 1, 2, 2];  // in semitones

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
    match key.chars().nth(0).unwrap() {
        'C' | 'D' | 'E' | 'F' | 'G' | 'A' | 'B' => {},
        _ => return Err("Wrong key value")
    }
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

fn notes_in_scale(scale: &Scale) -> Vec<&str> {
    let rule = match scale.scale_type {
        ScaleTypes::Maj => MAJOR_SCALE_RULE.iter(),
        ScaleTypes::Min => NAT_MIN_SCALE_RULE.iter(),
    };
    let mut piano = match scale.accidental {
        Accidentals::Sharp | Accidentals::Natural => SCALE_SHARPS,
        Accidentals::Flat => SCALE_FLATS,
    };

    let key_index = piano.iter().position(|&x| x == scale.key).unwrap();
    piano.rotate_left(key_index);

    let mut count = 0;
    let normal_rule: Vec<u8> = rule.map(|x| { count += x; count }).collect();

    let mut scale = vec![];
    scale.push(piano[0 as usize]);
    for &r in normal_rule[0..6].iter() {
        scale.push(piano[r as usize]);
    }
    scale
}

fn main() {
    println!("Scales and Chords little helper");
    let cli = Cli::parse();
    match cli.command {
        Commands::Scale { name } => {
            let scale_name: String = name.unwrap_or("Cmaj".to_string());
            let scale: Scale = parse_scale(&scale_name).unwrap();
            let notes: Vec<&str> = notes_in_scale(&scale);

            println!("Scale name: {:?}", scale_name);
            println!("Scale: {:#?}", scale);
            println!("Notes: {:?}", notes);
            // println!("Notes in scale {:?} : {:?}", scale_name, notes_in_scale(&scale_name));
        }
        Commands::Chords => {
            println!("Hello");
        }
    } 
}


#[cfg(test)]
mod tests {
    #[test]
    fn parse_ok_scales() {
        let scale = super::parse_scale("Amin").unwrap();
        assert_eq!(scale.key, "A");
        assert!(scale.accidental == super::Accidentals::Natural);
        assert!(scale.scale_type == super::ScaleTypes::Min);

        let scale = super::parse_scale("Cmaj").unwrap();
        assert_eq!(scale.key, "C");
        assert!(scale.accidental == super::Accidentals::Natural);
        assert!(scale.scale_type == super::ScaleTypes::Maj);

        let scale = super::parse_scale("D#min").unwrap();
        assert_eq!(scale.key, "D#");
        assert!(scale.accidental == super::Accidentals::Sharp);
        assert!(scale.scale_type == super::ScaleTypes::Min);

        let scale = super::parse_scale("Abmaj").unwrap();
        assert_eq!(scale.key, "Ab");
        assert!(scale.accidental == super::Accidentals::Flat);
        assert!(scale.scale_type == super::ScaleTypes::Maj);
    }

    #[test]
    fn parse_wrong_scale() {
        let scale = super::parse_scale("");
        assert!(scale.is_err());
        let scale = super::parse_scale("Xmin");
        assert!(scale.is_err());
        let scale = super::parse_scale("Something completely unrelated");
        assert!(scale.is_err());
        let scale = super::parse_scale("A$min");
        assert!(scale.is_err());
        let scale = super::parse_scale("A#foo");
        assert!(scale.is_err());
    }
}