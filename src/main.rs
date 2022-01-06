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
    Scale { name: Option<String> }, 
    Chord { name: Option<String> },
}

#[derive(Debug, PartialEq)]
enum Accidentals { Sharp, Flat, Natural }
#[derive(Debug, PartialEq)]
enum ScaleTypes { Maj, Min }
#[derive(Debug, PartialEq)]
enum ChordTypes { 
    Maj, 
    Min,
    Dim,
}

#[derive(Debug)]
struct Scale {
    key: String,
    accidental: Accidentals,
    scale_type: ScaleTypes,
}

#[derive(Debug)]
struct Chord {
    key: String, 
    accidental: Accidentals,
    chord_type: ChordTypes,
}


const SCALE_SHARPS: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const SCALE_FLATS: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

const MAJOR_SCALE_RULE: [u8; 7] = [2, 2, 1, 2, 2, 2, 1];  // in semitones
const NAT_MIN_SCALE_RULE: [u8; 7] = [2, 1, 2, 2, 1, 2, 2];  // in semitones

const CHORD_MAJ: [u8; 3] = [0, 4, 7];
const CHORD_MIN: [u8; 3] = [0, 3, 7];
const CHORD_DIM: [u8; 3] = [0, 3, 6];


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

    // figure out root note index and shift array to make it first element
    let key_index = piano.iter().position(|&x| x == scale.key).unwrap();
    piano.rotate_left(key_index);

    // convert rule to count indexes in scale instead if intervals
    let mut count = 0;
    let index_rule: Vec<u8> = rule.map(|x| { count += x; count }).collect();

    // extract scale using pre-calculated indexes
    let mut scale = vec![];
    scale.push(piano[0 as usize]);
    for &r in index_rule[0..6].iter() {
        scale.push(piano[r as usize]);
    }
    scale
}

fn parse_chord(chord_name: &str) -> Result<Chord, &str> {
    if chord_name.len() < 4 {
        return Err("Chord len is too small")
    }
    let accidental = match chord_name.chars().nth(1) {
        Some('#') => Accidentals::Sharp,
        Some('b') => Accidentals::Flat,
        _ => Accidentals::Natural
    };
    let key = match accidental {
        Accidentals::Sharp | Accidentals::Flat => chord_name[..2].to_string(),
        Accidentals::Natural => chord_name[..1].to_string() 
    };
    match key.chars().nth(0).unwrap() {
        'C' | 'D' | 'E' | 'F' | 'G' | 'A' | 'B' => {},
        _ => return Err("Wrong chord key value")
    }
    let chord_type_substr = match accidental {
        Accidentals::Sharp | Accidentals::Flat => chord_name[2..].to_string(),
        Accidentals::Natural => chord_name[1..].to_string()
    };
    let chord_type = match chord_type_substr.as_str() {
        "maj" => ChordTypes::Maj,
        "min" => ChordTypes::Min,
        "dim" => ChordTypes::Dim,
        _ => return Err("Wrong chord type")
    };
    Ok(Chord { key, accidental, chord_type })
}

fn notes_in_chord(chord: &Chord) -> Vec<&str> {
    let rule = match chord.chord_type {
        ChordTypes::Maj => CHORD_MAJ.iter(),
        ChordTypes::Min => CHORD_MIN.iter(),
        ChordTypes::Dim => CHORD_DIM.iter(),
    };
    let mut piano = match chord.accidental {
        Accidentals::Sharp | Accidentals::Natural => SCALE_SHARPS,
        Accidentals::Flat => SCALE_FLATS,
    };

    // figure out root note index and shift array to make it first element
    let key_index = piano.iter().position(|&x| x == chord.key).unwrap();
    piano.rotate_left(key_index);

    // extract scale using indexes in rule
    let mut chord = vec![];
    for &r in rule {
        chord.push(piano[r as usize]);
    }
    chord
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
            println!("Notes: {:?}", notes);
        }
        Commands::Chord { name } => {
            let chord_name: String = name.unwrap_or("Cmaj".to_string());
            let chord: Chord = parse_chord(&chord_name).unwrap();
            let notes: Vec<&str> = notes_in_chord(&chord);

            println!("Chord name: {:?}", chord_name);
            println!("Chord: {:?}", chord);
            println!("Notes: {:?}", notes);

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

        let scale = super::parse_scale("Cbmaj").unwrap();
        assert_eq!(scale.key, "Cb");
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