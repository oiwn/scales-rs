use crate::*;

#[derive(Debug, PartialEq)]
enum Accidentals {
    Sharp,
    Flat,
    Natural,
}

#[derive(Debug, PartialEq)]
enum ScaleTypes {
    Maj,
    Min,
}

#[derive(Debug, PartialEq)]
enum ChordTypes {
    Maj,
    Min,
    Dim,
}

#[derive(Debug, PartialEq)]
pub struct Scale {
    key: String,
    accidental: Accidentals,
    scale_type: ScaleTypes,
}

fn parse_key_acc(input: &str) -> Result<(String, Accidentals), String> {
    if input.len() < 4 {
        return Err(format!("Input '{}' is too short", input));
    }
    let accidental = match input.chars().nth(1) {
        Some('#') => Accidentals::Sharp,
        Some('b') => Accidentals::Flat,
        _ => Accidentals::Natural,
    };

    let key = match accidental {
        Accidentals::Sharp | Accidentals::Flat => input[..2].to_string(),
        Accidentals::Natural => input[..1].to_string(),
    };

    match key.chars().nth(0).unwrap() {
        'C' | 'D' | 'E' | 'F' | 'G' | 'A' | 'B' => {}
        _ => return Err(format!("Wrong key value '{}'", key)),
    }

    Ok((key, accidental))
}

pub fn parse_scale(scale_name: &str) -> Result<Scale, String> {
    let (key, accidental) = parse_key_acc(scale_name)?;
    let scale_type_substr = match accidental {
        Accidentals::Sharp | Accidentals::Flat => scale_name[2..].to_string(),
        Accidentals::Natural => scale_name[1..].to_string(),
    };
    let scale_type = match scale_type_substr.as_str() {
        "maj" => ScaleTypes::Maj,
        "min" => ScaleTypes::Min,
        _ => return Err(format!("Error parsing scale '{}', wrong scale type", scale_name)),
    };
    Ok(Scale {
        key,
        accidental,
        scale_type,
    })
}

pub fn notes_in_scale(scale: &Scale) -> Vec<&str> {
    let rule = match scale.scale_type {
        ScaleTypes::Maj => consts::SCALE_MAJOR,
        ScaleTypes::Min => consts::SCALE_NAT_MINOR,
    };
    let mut piano = match scale.accidental {
        Accidentals::Sharp | Accidentals::Natural => consts::NOTES_SHARPS,
        Accidentals::Flat => consts::NOTES_FLATS,
    };

    // figure out root note index and shift array to make it first element
    let key_index = piano.iter().position(|&x| x == scale.key).unwrap();
    piano.rotate_left(key_index);

    // convert rule to count indexes in scale instead if intervals
    let mut count = 0;
    let index_rule: Vec<u8> = rule.iter()
        .map(|x| {
            count += x;
            count
        })
        .collect();

    // extract scale using pre-calculated indexes
    let mut scale = vec![];
    scale.push(piano[0 as usize]);
    for &r in index_rule[0..6].iter() {
        scale.push(piano[r as usize]);
    }
    scale
}

#[derive(Debug)]
pub struct Chord {
    key: String,
    accidental: Accidentals,
    chord_type: ChordTypes,
}

pub fn parse_chord(chord_name: &str) -> Result<Chord, String> {
    let (key, accidental) = parse_key_acc(chord_name)?;
    let chord_type_substr = match accidental {
        Accidentals::Sharp | Accidentals::Flat => chord_name[2..].to_string(),
        Accidentals::Natural => chord_name[1..].to_string(),
    };
    let chord_type = match chord_type_substr.as_str() {
        "maj" => ChordTypes::Maj,
        "min" => ChordTypes::Min,
        "dim" => ChordTypes::Dim,
        _ => return Err(format!("Wrong chord type: '{}'", chord_name)),
    };
    Ok(Chord {
        key,
        accidental,
        chord_type,
    })
}

pub fn notes_in_chord(chord: &Chord) -> Vec<&str> {
    let rule = match chord.chord_type {
        ChordTypes::Maj => consts::CHORD_MAJ,
        ChordTypes::Min => consts::CHORD_MIN,
        ChordTypes::Dim => consts::CHORD_DIM,
    };
    let mut piano = match chord.accidental {
        Accidentals::Sharp | Accidentals::Natural => consts::NOTES_SHARPS,
        Accidentals::Flat => consts::NOTES_FLATS,
    };

    // figure out root note index and shift array to make it first element
    let key_index = piano.iter().position(|&x| x == chord.key).unwrap();
    piano.rotate_left(key_index);

    // extract scale using indexes in rule
    let mut chord = vec![];
    for &r in rule.iter() {
        chord.push(piano[r as usize]);
    }
    chord
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_key_acc_ok() {
        let result = super::parse_key_acc("Amin").unwrap();
        assert_eq!(result, (String::from("A"), super::Accidentals::Natural));

        let result = super::parse_key_acc("Cmaj").unwrap();
        assert_eq!(result, (String::from("C"), super::Accidentals::Natural));

        let result = super::parse_key_acc("D#min").unwrap();
        assert_eq!(result, (String::from("D#"), super::Accidentals::Sharp));

        let result = super::parse_key_acc("Abmaj").unwrap();
        assert_eq!(result, (String::from("Ab"), super::Accidentals::Flat));
    }

    #[test]
    fn parse_key_acc_err() {
        let result = super::parse_key_acc("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), String::from("Input '' is too short"));

        let result = super::parse_key_acc("Xmin");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), String::from("Wrong key value 'X'"));
    }

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
