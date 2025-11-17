use midly::stream::MidiStream;
use midly::MidiMessage;

struct MidiSurface {
    midi_in: MidiStream,
}

impl MidiSurface {
    fn new() -> Self {
        Self {
            midi_in: MidiStream::new(),
        }
    }
}
