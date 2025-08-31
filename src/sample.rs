
use rodio::source::{Source};
use std::time::Duration;

const DEFAULT_SAMPLE_RATE: f32 = 44100.0;

pub struct RawAudioDataSource {
	samples: Vec<f32>,
	current: usize
}

impl RawAudioDataSource {

	pub fn new(samples: Vec<f32>, current: usize) -> Self {
		Self {samples, current}
	}

	pub fn set_samples(&mut self, samples: Vec<f32>) {
		self.samples = samples;
	}

}

impl Iterator for RawAudioDataSource {
	type Item = f32;
	
	fn next(&mut self) -> Option<f32> {
		if self.current < self.samples.len() {
			let sample = self.samples[self.current];
			self.current += 1;
			Some(sample)
		} else {
			None
		}
	}
}

impl Source for RawAudioDataSource {
	fn current_span_len(&self) -> Option<usize> {
		Some(self.samples.len() - self.current)
	}

	fn channels(&self) -> u16 {
		1
	}
	fn sample_rate(&self) -> u32 {
		DEFAULT_SAMPLE_RATE as u32
	}

	fn total_duration(&self) -> Option<Duration> {
		Some(Duration::from_secs_f32(self.samples.len() as f32 / DEFAULT_SAMPLE_RATE))
	}
}
