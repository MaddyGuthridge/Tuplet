/// A musical note
///
/// While it's closer to the MIDI spec to represent a note as a note on/off
/// pair, this is considerably more painful to work with musically, and so this
/// conversion should be performed only when converting to/from MIDI.
///
/// Where reasonable, this struct is designed to support MIDI 2 capabilities.
/// In particular, values such as velocity are represented using 16 bits.
#[derive(Debug, Clone)]
pub struct Note {
    /// The start position of this note within its parent container, measured
    /// in ticks
    start: u32,
    /// The duration of this note, measured in ticks
    duration: u32,
    /// The pitch of the note, in MIDI standard format (ie 60 = middle C)
    note_number: u8,
    /// The note-on velocity of the note
    velocity: u16,
    /// The note-off velocity of the note
    off_velocity: u16,
    /// The portamento origin note.
    ///
    /// This is represented by a control change 84 in a previous event. See
    /// section 7.4.6.1 of MIDI 2 Protocol Spec.
    portamento_source_node: Option<u8>,
}

impl Note {
    /// Create a new note with the given properties
    pub fn new(start: u32, duration: u32, note_number: u8, velocity: u16, off_velocity: u16) -> Note {
        Note {
            start,
            duration,
            note_number,
            velocity,
            off_velocity,
            portamento_source_node: None,
        }
    }

    /// Return a MIDI 1 format note-on event for this note
    pub fn midi_1_note_on(&self, channel: u8) -> [u8; 3] {
        let scaled_velocity: u8 = (self.velocity / 0xFF).try_into().unwrap();
        [
            0x90 + channel,
            self.note_number,
            // Velocity zero represents a note-off event, so we must ensure
            // any zero-velocity notes are increased to velocity 1
            if scaled_velocity == 0 { scaled_velocity } else { 1 },
        ]
    }
}
