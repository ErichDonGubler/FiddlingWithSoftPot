use sample::{
    signal::{self, ConstHz, Sine},
    Signal,
};
use serial::prelude::*;
use serial::ErrorKind::NoDevice;

use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

// use rand::Rng;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{StreamData, UnknownTypeOutputBuffer};

fn main() {
    // Set up port to get data from arduino
    let mut port = match serial::open("COM4") {
        Ok(result) => result,
        Err(error) => {
            if error.kind() == NoDevice {
                println!("No device was found!");
            }
            return;
        }
    };
    port.reconfigure(&|settings| settings.set_baud_rate(serial::Baud9600))
        .unwrap();

    // Set up audio stuff
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

    const NUM_NOTES_IN_CHROMATIC_SCALE: usize = 12;
    let mut notes: [Sine<ConstHz>; NUM_NOTES_IN_CHROMATIC_SCALE] = [
        signal::rate(44_100.0).const_hz(261.626).sine(), // C
        signal::rate(44_100.0).const_hz(277.183).sine(), // C#
        signal::rate(44_100.0).const_hz(293.665).sine(), // D
        signal::rate(44_100.0).const_hz(311.127).sine(), // D#
        signal::rate(44_100.0).const_hz(329.628).sine(), // E
        signal::rate(44_100.0).const_hz(349.228).sine(), // F
        signal::rate(44_100.0).const_hz(369.994).sine(), // F#
        signal::rate(44_100.0).const_hz(391.995).sine(), // G
        signal::rate(44_100.0).const_hz(415.305).sine(), // G#
        signal::rate(44_100.0).const_hz(440.000).sine(), // A
        signal::rate(44_100.0).const_hz(466.164).sine(), // A#
        signal::rate(44_100.0).const_hz(493.883).sine(), // B
    ];

    let current_note = Arc::new(Mutex::new(0));
    let get_note = Arc::clone(&current_note);
    let play_note = Arc::clone(&current_note);
    thread::spawn(move || {
        loop {
            let mut slice = [0; 2];
            let resultythingy = port.read_exact(&mut slice[..]);
            match resultythingy {
                Ok(()) => {
                    // println!("                                      Everything is ok!");
                    let read_note = (i32::from(slice[0] % b'0') * 10) + i32::from(slice[1] % b'0');
                    let mut note = get_note.lock().unwrap();
                    if read_note != *note {
                        *note = read_note
                    }
                    // println!("                                          Note! {:?}", *note)
                }
                Err(_resultythingy) => {
                    // println!("                                      Oh no! {:?}", resultythingy);
                }
            }
        }
    });
    event_loop.run(move |_stream_id, _stream_result| {
        let note = {
            // This unlocks the mutex before the loop so the other thread can use it faster.
            *play_note.lock().unwrap()
        };
        println!("reading in event note: {}", note);
        let stream_data = _stream_result.unwrap();
        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                // I was fiddling with static as a test before I figured out specific notes.
                // let mut rng = rand::thread_rng();
                for elem in buffer.iter_mut() {
                    const NUM_NOTES_IN_CHROMATIC_SCALE_I32: i32 =
                        NUM_NOTES_IN_CHROMATIC_SCALE as i32;
                    // *elem = rng.gen::<f32>()
                    let next_value = match note {
                        // notes is a slice that has the first note at idx 0.
                        idx @ 1..=NUM_NOTES_IN_CHROMATIC_SCALE_I32 => notes[(idx-1) as usize].next()[0],
                        _ => 0.0,
                    };
                    *elem = (next_value / 5.0) as f32;
                }
            }
            _ => (),
        }
    });
}
