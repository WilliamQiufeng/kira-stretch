use std::{
    path::PathBuf, time::Duration
};

use clap::{command, Parser};
use console::Term;
use kira::{
    modulator::tweener::TweenerBuilder, sound::static_sound::StaticSoundData, track::TrackBuilder, AudioManager, DefaultBackend, Mapping, Semitones, Tween, Value
};
use kira_stretch::effect::pitch::PitcherBuilder;



#[derive(Parser, Debug)]
#[command(name = "stretch")]
struct Args {
    /// The file to play.
    file: PathBuf,
}

fn tween() -> Tween {
    Tween {
        duration: Duration::from_millis(500),
        easing: kira::Easing::Linear,
        ..Default::default()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut manager = AudioManager::<DefaultBackend>::new(Default::default())?;
    // Create a mixer sub-track with a filter.
    let mut pitch = Semitones(0.0);
    let mut playback_rate = Semitones(0.0);

    let mut tweener = manager.add_modulator(TweenerBuilder { initial_value: 0.0 })?;

    const SEMITONE_STEP: Semitones = Semitones(1.0);
    let mut track = manager.add_sub_track({
        let mut builder = TrackBuilder::new();
        builder.add_effect(PitcherBuilder::new().pitch(Value::from_modulator(&tweener, Mapping {
            input_range: (-12.0, 12.0),
            output_range: (-12.0, 12.0),
            easing: kira::Easing::Linear,
        })));
        builder
    })?;
    // Play the sound on the track.
    let sound_data = StaticSoundData::from_file(args.file)?;
    // let tonality_limit = Some(8000.0 / sound_data.sample_rate as f32);
    // effect.set_tonality_limit(tonality_limit);
    let mut sound = track.play(sound_data)?;
    

    let term = Term::stdout();
    println!("Press 'w' to increase pitch, 's' to decrease pitch, 'a' to decrease playback rate, 'd' to increase playback rate.");
    loop {
        println!("Pitch: {:?}, Playback Rate: {:?}", pitch, playback_rate);
        let input = term.read_char()?;
        // input wasd keys and adjust pitch and playback rate
        match input {
            'w' => {
                pitch += SEMITONE_STEP;
                tweener.set(pitch.0, tween());
            }
            's' => {
                pitch -= SEMITONE_STEP;
                tweener.set(pitch.0, tween());
            }
            'a' => {
                playback_rate -= SEMITONE_STEP;
                sound.set_playback_rate(playback_rate, tween());
            }
            'd' => {
                playback_rate += SEMITONE_STEP;
                sound.set_playback_rate(playback_rate, tween());
            }
            _ => {}
        }
    }
}
