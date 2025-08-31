
// Distribute freely, do not commercialize and sell
// nsk 2025

#![allow(warnings)]

use std::io;
use x11::xlib;
use std::ptr;

use rodio::{OutputStream, Sink, Source};
use rodio::source::{SineWave};
use std::time::Duration;

use std::f32::consts::PI;
use std::collections::HashMap;

extern crate x11;
extern crate meval;

use std::cell::Cell;
use meval::{Expr, Context};


mod sample;

fn sine_wave(t: f32, freq: f32, rate: usize) -> f32 {
	f32::sin(t / rate as f32 * freq)
}

fn print_info(version: &str)
{
 	println!("** {version} ** \nl(oad) <raw audio file> <target instrument>\n");
	println!("p(lay) <instrument> <duration>");
	println!("c(reate) <expression> <sample rate>");
        println!("variables: rate, grad");

	println!("e(xit) the program");
}

fn tovec32(vector: Vec<f64>) -> Vec<f32> {
    vector.iter().map(|&i| i as f32).collect()
}

fn main() {
    print_info("nsk-daw 1v0831");
    
    const DEFAULT_SAMPLE_RATE: usize = 44100;

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("[ERROR] Failure to open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    let mut instruments: HashMap<String, String> = HashMap::new();

    let sample_duration: usize      = 2;
    let mut sample_size: usize      = DEFAULT_SAMPLE_RATE * sample_duration;
    let mut audio_data: Vec<f32>    = vec![0.0; sample_size];  

    loop {

        let mut user_input = String::new();
        match std::io::stdin().read_line(&mut user_input) {
            Ok(_) => {},
            Err(e) => { eprintln!("[ERROR] {:?}", e) }
        }

        let tokens: Vec<&str> = user_input.split_whitespace().collect();

        match tokens.get(0).map(|s| s.trim()) {
            Some("create") => {

                sink.clear();

                let expr : meval::Expr = tokens.get(1).unwrap().parse().unwrap();
                let sample_size: usize = meval::eval_str(tokens.get(2).unwrap())
                    .unwrap()
                    .round() as usize;

                let mut ctx = Context::new();
                ctx.var("rate", DEFAULT_SAMPLE_RATE as f64);
                ctx.func("period", |x| x / DEFAULT_SAMPLE_RATE as f64 * std::f64::consts::PI*2.0);


                let func = expr.bind_with_context(ctx, "t").unwrap();
                let source = sample::RawAudioDataSource::new( tovec32( (0..sample_size+1).map(|t| func(t as f64)).collect() ), 0 );
                sink.append(source);
                sink.pause();
                println!("created a sample with a length of {} seconds", sample_size / DEFAULT_SAMPLE_RATE);
            },
            Some("play") => {
                match sink.try_seek(std::time::Duration::from_secs(0)) {
                    Ok(_) => println!("Reset audio position to 0"),
                    Err(e) => eprintln!("[ERROR] Failed: {:?}", e),
                }
                sink.play();
            },
            Some("exit") => {
                break;
            },
            None => println!("asd"),
            _ => println!(""),
        }
    }
}
