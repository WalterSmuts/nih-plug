use crate::context::process::Transport;
use crate::midi::PluginNoteEvent;

mod cpal;
mod dummy;
mod jack;

pub use self::cpal::Cpal;
pub use self::dummy::Dummy;
pub use self::jack::Jack;
pub use crate::buffer::Buffer;
pub use crate::plugin::Plugin;

/// An audio+MIDI backend for the standalone wrapper.
pub trait Backend<P: Plugin>: 'static + Send + Sync {
    /// Start processing audio and MIDI on this thread. The process callback will be called whenever
    /// there's a new block of audio to be processed. The process callback receives the audio
    /// buffers for the wrapped plugin's outputs. Any inputs will have already been copied to this
    /// buffer. This will block until the process callback returns `false`.
    ///
    /// TODO: Auxiliary inputs and outputs
    fn run(
        &mut self,
        cb: impl FnMut(
                &mut Buffer,
                Transport,
                &[PluginNoteEvent<P>],
                &mut Vec<PluginNoteEvent<P>>,
            ) -> bool
            + 'static
            + Send,
    );
}
