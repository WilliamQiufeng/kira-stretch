//! Changes the pitch of the sound without changing the playback rate.

pub mod builder;
pub mod handle;

pub use builder::*;
pub use handle::*;

use kira::{
    Frame, Parameter, command::ValueChangeCommand, command_writers_and_readers,
    effect::Effect, info::Info,
};
macro_rules! read_commands_into_parameters {
	($self:ident, $($parameter_name:ident),*$(,)?) => {
		paste::paste! {
			$($self.$parameter_name.read_command(&mut $self.command_readers.[<set_ $parameter_name>]);)*
		}
	};
}

struct Pitcher {
    command_readers: CommandReaders,
    pitch: Parameter,
    stretch: Option<signalsmith_stretch::Stretch>,
    input_buffer: Vec<f32>,
    tonality_limit: Option<f32>,
}

impl Pitcher {
    #[must_use]
    fn new(builder: PitcherBuilder, command_readers: CommandReaders) -> Self {
        Self {
            command_readers,
            pitch: Parameter::new(builder.pitch, 0.0),
            stretch: None,
            input_buffer: Vec::new(),
            tonality_limit: builder.tonality_limit,
        }
    }
}

struct MutInterlacedSamples<'a>(pub &'a mut [Frame]);
impl AsMut<[f32]> for MutInterlacedSamples<'_> {
    fn as_mut(&mut self) -> &mut [f32] {
        unsafe { std::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut f32, self.0.len() * 2) }
    }
}
impl AsRef<[f32]> for MutInterlacedSamples<'_> {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const f32, self.0.len() * 2) }
    }
}
struct RefInterlacedSamples<'a>(pub &'a [Frame]);
impl AsRef<[f32]> for RefInterlacedSamples<'_> {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const f32, self.0.len() * 2) }
    }
}

impl Effect for Pitcher {
    fn on_start_processing(&mut self) {
        if let Some(tonality_limit) = self.command_readers.set_tonality_limit.read() {
            self.tonality_limit = tonality_limit;
        }
        read_commands_into_parameters!(self, pitch);
    }

    fn process(&mut self, input: &mut [Frame], dt: f64, info: &Info) {
        self.pitch.update(dt * input.len() as f64, info);
        self.stretch
            .as_mut()
            .unwrap()
            .set_transpose_factor_semitones(self.pitch.value() as f32, self.tonality_limit);

        let b = RefInterlacedSamples(input);
        let input_len = b.as_ref().len();
        self.input_buffer.resize(input_len, 0.0);
        self.input_buffer.copy_from_slice(b.as_ref());
        self.stretch
            .as_mut()
            .unwrap()
            .process(&self.input_buffer, MutInterlacedSamples(input));
    }

    fn init(&mut self, sample_rate: u32, _internal_buffer_size: usize) {
        let _ = self
            .stretch
            .insert(signalsmith_stretch::Stretch::preset_default(2, sample_rate));
    }
}

command_writers_and_readers!(
    set_pitch: ValueChangeCommand<f64>,
    set_tonality_limit: Option<f32>,
);
