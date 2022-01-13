/* Implementation of scales and chords in forma of library */

/* Constants and rules for chords / scales / notes */

// Notes - sharps and flats

pub const NOTES_SHARPS: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
pub const NOTES_FLATS: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

// Scales rules in semitone intervals between notes

pub const SCALE_MAJOR: [u8; 7] = [2, 2, 1, 2, 2, 2, 1];
pub const SCALE_NAT_MINOR: [u8; 7] = [2, 1, 2, 2, 1, 2, 2];

// Rules for chords
// https://en.wikipedia.org/wiki/Major_chord
// https://en.wikipedia.org/wiki/Minor_chord
// https://en.wikipedia.org/wiki/Diminished_triad
// https://en.wikipedia.org/wiki/Augmented_triad
// https://en.wikipedia.org/wiki/Dominant_seventh_chord
// https://en.wikipedia.org/wiki/Minor_seventh_chord
// https://en.wikipedia.org/wiki/Major_seventh_chord
// https://en.wikipedia.org/wiki/Suspended_chord

pub const CHORD_MAJ: [u8; 3] = [0, 4, 7];
pub const CHORD_MIN: [u8; 3] = [0, 3, 7];
pub const CHORD_DIM: [u8; 3] = [0, 3, 6];
pub const CHORD_AUG: [u8; 3] = [0, 4, 8];

pub const CHORD_DOM7: [u8; 4] = [0, 4, 7, 10];
pub const CHORD_MIN7: [u8; 4] = [0, 3, 7, 10];
pub const CHORD_MAJ7: [u8; 4] = [0, 4, 7, 11];

pub const CHORD_SUS2: [u8; 3] = [0, 2, 7];
pub const CHORD_SUS4: [u8; 3] = [0, 5, 7];

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
    Aug,
    Dom7,
    Min7,
    Maj7,
    Sus2,
    Sus4,
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

#[derive(Debug, PartialEq)]
pub struct Scale {
    key: String,
    accidental: Accidentals,
    scale_type: ScaleTypes,
}

impl Scale {
    pub fn parse(scale_name: &str) -> Result<Scale, String> {
        let (key, accidental) = parse_key_acc(scale_name)?;
        let scale_type_substr = match accidental {
            Accidentals::Sharp | Accidentals::Flat => scale_name[2..].to_string(),
            Accidentals::Natural => scale_name[1..].to_string(),
        };
        let scale_type = match scale_type_substr.as_str() {
            "maj" => ScaleTypes::Maj,
            "min" => ScaleTypes::Min,
            _ => {
                return Err(format!(
                    "Error parsing scale '{}', wrong scale type",
                    scale_name
                ))
            }
        };
        Ok(Scale {
            key,
            accidental,
            scale_type,
        })
    }

    pub fn to_notes(&self) -> Vec<&str> {
        let rule = match self.scale_type {
            ScaleTypes::Maj => SCALE_MAJOR,
            ScaleTypes::Min => SCALE_NAT_MINOR,
        };
        let mut piano = match self.accidental {
            Accidentals::Sharp | Accidentals::Natural => NOTES_SHARPS,
            Accidentals::Flat => NOTES_FLATS,
        };

        // figure out root note index and shift array to make it first element
        let key_index = piano.iter().position(|&x| x == self.key).unwrap();
        piano.rotate_left(key_index);

        // convert rule to count indexes in scale instead if intervals
        let mut count = 0;
        let index_rule: Vec<u8> = rule
            .iter()
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

    pub fn to_string(&self) -> String {
        self.to_notes().join("-")
    }
}

#[derive(Debug)]
pub struct Chord {
    key: String,
    accidental: Accidentals,
    chord_type: ChordTypes,
}

impl Chord {
    pub fn parse(chord_name: &str) -> Result<Chord, String> {
        let (key, accidental) = parse_key_acc(chord_name)?;
        let chord_type_substr = match accidental {
            Accidentals::Sharp | Accidentals::Flat => chord_name[2..].to_string(),
            Accidentals::Natural => chord_name[1..].to_string(),
        };
        let chord_type = match chord_type_substr.as_str() {
            "maj" => ChordTypes::Maj,
            "min" => ChordTypes::Min,
            "dim" => ChordTypes::Dim,
            "aug" => ChordTypes::Aug,
            "dom7" => ChordTypes::Dom7,
            "min7" => ChordTypes::Min7,
            "maj7" => ChordTypes::Maj7,
            "sus2" => ChordTypes::Sus2,
            "sus4" => ChordTypes::Sus4,
            _ => return Err(format!("Wrong chord type: '{}'", chord_name)),
        };
        Ok(Chord {
            key,
            accidental,
            chord_type,
        })
    }

    pub fn to_notes(&self) -> Vec<&str> {
        let rule = match self.chord_type {
            ChordTypes::Maj => CHORD_MAJ.to_vec(),
            ChordTypes::Min => CHORD_MIN.to_vec(),
            ChordTypes::Dim => CHORD_DIM.to_vec(),
            ChordTypes::Aug => CHORD_AUG.to_vec(),
            ChordTypes::Dom7 => CHORD_DOM7.to_vec(),
            ChordTypes::Min7 => CHORD_MIN7.to_vec(),
            ChordTypes::Maj7 => CHORD_MAJ7.to_vec(),
            ChordTypes::Sus2 => CHORD_SUS2.to_vec(),
            ChordTypes::Sus4 => CHORD_SUS4.to_vec(),
        };
        let mut piano = match self.accidental {
            Accidentals::Sharp | Accidentals::Natural => NOTES_SHARPS,
            Accidentals::Flat => NOTES_FLATS,
        };

        // figure out root note index and shift array to make it first element
        let key_index = piano.iter().position(|&x| x == self.key).unwrap();
        piano.rotate_left(key_index);

        // extract scale using indexes in rule
        let mut chord = vec![];
        for &r in rule.iter() {
            chord.push(piano[r as usize]);
        }
        chord
    }
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
        assert_eq!(
            result.unwrap_err().to_string(),
            String::from("Input '' is too short")
        );

        let result = super::parse_key_acc("Xmin");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            String::from("Wrong key value 'X'")
        );
    }

    #[test]
    fn parse_ok_scales() {
        let scale = super::Scale::parse("Amin").unwrap();
        assert_eq!(scale.key, "A");
        assert!(scale.accidental == super::Accidentals::Natural);
        assert!(scale.scale_type == super::ScaleTypes::Min);

        let scale = super::Scale::parse("Cmaj").unwrap();
        assert_eq!(scale.key, "C");
        assert!(scale.accidental == super::Accidentals::Natural);
        assert!(scale.scale_type == super::ScaleTypes::Maj);

        let scale = super::Scale::parse("D#min").unwrap();
        assert_eq!(scale.key, "D#");
        assert!(scale.accidental == super::Accidentals::Sharp);
        assert!(scale.scale_type == super::ScaleTypes::Min);

        let scale = super::Scale::parse("Abmaj").unwrap();
        assert_eq!(scale.key, "Ab");
        assert!(scale.accidental == super::Accidentals::Flat);
        assert!(scale.scale_type == super::ScaleTypes::Maj);

        let scale = super::Scale::parse("Cbmaj").unwrap();
        assert_eq!(scale.key, "Cb");
        assert!(scale.accidental == super::Accidentals::Flat);
        assert!(scale.scale_type == super::ScaleTypes::Maj);
    }

    #[test]
    fn parse_wrong_scales() {
        let scale = super::Scale::parse("");
        assert!(scale.is_err());
        let scale = super::Scale::parse("Xmin");
        assert!(scale.is_err());
        let scale = super::Scale::parse("Something completely unrelated");
        assert!(scale.is_err());
        let scale = super::Scale::parse("A$min");
        assert!(scale.is_err());
        let scale = super::Scale::parse("A#foo");
        assert!(scale.is_err());
    }

    #[test]
    fn parse_ok_chords() {
        let chord = super::Chord::parse("Amin").unwrap();
        assert_eq!(chord.key, "A");
        assert!(chord.accidental == super::Accidentals::Natural);
        assert!(chord.chord_type == super::ChordTypes::Min);

        let chord = super::Chord::parse("Cmaj").unwrap();
        assert_eq!(chord.key, "C");
        assert!(chord.accidental == super::Accidentals::Natural);
        assert!(chord.chord_type == super::ChordTypes::Maj);

        let chord = super::Chord::parse("D#min").unwrap();
        assert_eq!(chord.key, "D#");
        assert!(chord.accidental == super::Accidentals::Sharp);
        assert!(chord.chord_type == super::ChordTypes::Min);

        let chord = super::Chord::parse("Abmaj").unwrap();
        assert_eq!(chord.key, "Ab");
        assert!(chord.accidental == super::Accidentals::Flat);
        assert!(chord.chord_type == super::ChordTypes::Maj);

        let chord = super::Chord::parse("Cbmaj").unwrap();
        assert_eq!(chord.key, "Cb");
        assert!(chord.accidental == super::Accidentals::Flat);
        assert!(chord.chord_type == super::ChordTypes::Maj);
    }

    #[test]
    fn parse_wrong_chords() {
        let chord = super::Chord::parse("");
        assert!(chord.is_err());
        let chord = super::Chord::parse("Xmin");
        assert!(chord.is_err());
        let chord = super::Chord::parse("Something completely unrelated");
        assert!(chord.is_err());
        let chord = super::Chord::parse("A$min");
        assert!(chord.is_err());
        let chord = super::Chord::parse("A#foo");
        assert!(chord.is_err());
    }
}
