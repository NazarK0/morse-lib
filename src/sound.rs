use std::time::Duration;

use rodio::{source::SineWave, OutputStream, Sink, Source};

#[derive(Debug, PartialEq, Clone)]
pub struct Sound {
    pub frequency: f32,
    pub speed: f32,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            frequency: 450.0,
            speed: 1.0,
        }
    }
}

impl TSound for Sound {}

pub trait TSound {
    fn play(&self, freq: f32, duration: u8, speed: f32) {
        // on linux require pkg-config libudev-dev libasound2-dev
        // _stream must live as long as the sink
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let source = SineWave::new(freq)
            .take_duration(Duration::from_secs_f32((duration as f32) / speed))
            .amplify(0.20);
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    }
}
