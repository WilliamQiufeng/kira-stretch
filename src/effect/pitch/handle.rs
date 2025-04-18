
use super::CommandWriters;

/// Controls a filter effect.
#[derive(Debug)]
pub struct PitcherHandle {
	pub(super) command_writers: CommandWriters,
}
macro_rules! handle_param_setters {
	($($(#[$m:meta])* $name:ident: $type:ty),*$(,)?) => {
		paste::paste! {
			$(
				$(#[$m])*
				pub fn [<set_ $name>](&mut self, $name: impl Into<kira::Value<$type>>, tween: kira::Tween) {
					self.command_writers.[<set_ $name>].write(kira::command::ValueChangeCommand {
						target: $name.into(),
						tween,
					})
				}
			)*
		}
	};
}
impl PitcherHandle {
    /// Sets the frequencies that the filter will remove.
	pub fn set_tonality_limit(&mut self, tonality_limit: Option<f32>) {
		self.command_writers.set_tonality_limit.write(tonality_limit);
	}
	handle_param_setters! {
		pitch: f64
	}
}