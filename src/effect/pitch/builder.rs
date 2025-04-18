use kira::{
	effect::{Effect, EffectBuilder}, Value,
};

use super::{command_writers_and_readers, PitcherHandle, Pitcher};

/// Configures a pitcher effect.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PitcherBuilder {
	/// The pitch of the pitcher.
	pub pitch: Value<f64>,

    /// The tonality limit of the pitcher.
    pub tonality_limit: Option<f32>,
}

impl PitcherBuilder {
	/// Creates a new [`PitcherBuilder`] with the default settings.
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

    /// Sets the tonality limit of the pitcher.
    #[must_use = "This method consumes self and returns a modified PitcherBuilder, so the return value should be used"]
    pub fn tonality_limit(self, tonality_limit: Option<f32>) -> Self {
        Self {
            tonality_limit,
            ..self
        }
    }

	/// Sets the resonance of the pitcher.
	#[must_use = "This method consumes self and returns a modified PitcherBuilder, so the return value should be used"]
	pub fn pitch(self, pitch: impl Into<Value<f64>>) -> Self {
		Self {
			pitch: pitch.into(),
			..self
		}
	}
}

impl Default for PitcherBuilder {
	fn default() -> Self {
		Self {
			pitch: Value::Fixed(0.0),
            tonality_limit: None,
		}
	}
}

impl EffectBuilder for PitcherBuilder {
	type Handle = PitcherHandle;

	fn build(self) -> (Box<dyn Effect>, Self::Handle) {
		let (command_writers, command_readers) = command_writers_and_readers();
		(
			Box::new(Pitcher::new(self, command_readers)),
			PitcherHandle { command_writers },
		)
	}
}