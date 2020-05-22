//use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
//use cpal::{StreamData, UnknownTypeOutputBuffer};
use hound;

/*
fn hound_test() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}

fn cpal() {
    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_formats_range = device
        .supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range
        .next()
        .expect("no supported format?!")
        .with_max_sample_rate();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop
        .play_stream(stream_id)
        .expect("failed to play_stream");

    event_loop.run(move |stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                return;
            }
        };

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = u16::max_value() / 2;
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = 0;
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = 0.0;
                }
            }
            _ => (),
        }
    });
}
*/
fn append_square_wave(samples: &mut Vec<f32>, wave_length: f32) {
    for index in 0..wave_length as u32 {
        if index < (wave_length / 2.0) as u32 {
            samples.push(-1.0)
        } else {
            samples.push(1.0)
        }
    }
}

fn append_square_wave_freq(samples: &mut Vec<f32>, frequency: f32, seconds: f32, sample_rate: f32) {
    let peroid: f32 = 1.0 / frequency;
    let wave_length: f32 = peroid * sample_rate;
    let samples_count: f32 = seconds * sample_rate;
    let waves_count: f32 = (samples_count/ wave_length).ceil();
    for _index in 0..waves_count as u32 {
        append_square_wave(samples, wave_length);
    }

}

fn main() {
    let sample_rate: f32 = 48000.0;
    let mut samples: Vec<f32> = Vec::new();
    append_square_wave_freq(&mut samples, 16.0, 8.0, sample_rate);
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create("sonido.wav", spec).unwrap();
    for sample in &samples {
        writer.write_sample(*sample).unwrap();
    }
    writer.finalize().unwrap();
}
