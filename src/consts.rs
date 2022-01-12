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

