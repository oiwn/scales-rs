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

pub const CHORD_MAJ: [u8; 3] = [0, 4, 7];
pub const CHORD_MIN: [u8; 3] = [0, 3, 7];
pub const CHORD_DIM: [u8; 3] = [0, 3, 6];
