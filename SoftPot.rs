use sample::{signal, Signal};
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

    // Define each note
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    let mut C      = signal::rate(44_100.0).const_hz(261.626).sine();
    #[allow(non_snake_case)]
    let mut Csharp = signal::rate(44_100.0).const_hz(277.183).sine();
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    let mut D      = signal::rate(44_100.0).const_hz(293.665).sine();
    #[allow(non_snake_case)]
    let mut Dsharp = signal::rate(44_100.0).const_hz(311.127).sine();
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    let mut E      = signal::rate(44_100.0).const_hz(329.628).sine();
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    let mut F      = signal::rate(44_100.0).const_hz(349.228).sine();
    #[allow(non_snake_case)]
    let mut Fsharp = signal::rate(44_100.0).const_hz(369.994).sine();
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    let mut G      = signal::rate(44_100.0).const_hz(391.995).sine();
    #[allow(non_snake_case)]
    let mut Gsharp = signal::rate(44_100.0).const_hz(415.305).sine();
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    let mut A      = signal::rate(44_100.0).const_hz(440.000).sine();
    #[allow(non_snake_case)]
    let mut Asharp = signal::rate(44_100.0).const_hz(466.164).sine();
    #[allow(non_snake_case)]
    #[rustfmt::skip]
    let mut B      = signal::rate(44_100.0).const_hz(493.883).sine();

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
                    // The arduino sends ascii, and 0 in ascii is 48.
                    let read_note = (i32::from(slice[0] % 48) * 10) + i32::from(slice[1] % 48);
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
        // This unlocks the mutex before the loop so the other thread can use it faster.
        let note = { *play_note.lock().unwrap() };
        println!("reading in event note: {}", note);
        let stream_data = _stream_result.unwrap();
        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                // I was fiddling with static as a test before I figured out specific notes.
                // let mut rng = rand::thread_rng();
                for elem in buffer.iter_mut() {
                    // *elem = rng.gen::<f32>()
                    let next_value;
                    match note {
                        #[rustfmt::skip]
                        1  => next_value = C.next()[0],
                        #[rustfmt::skip]
                        2  => next_value = Csharp.next()[0],
                        #[rustfmt::skip]
                        3  => next_value = D.next()[0],
                        #[rustfmt::skip]
                        4  => next_value = Dsharp.next()[0],
                        #[rustfmt::skip]
                        5  => next_value = E.next()[0],
                        #[rustfmt::skip]
                        6  => next_value = F.next()[0],
                        #[rustfmt::skip]
                        7  => next_value = Fsharp.next()[0],
                        #[rustfmt::skip]
                        8  => next_value = G.next()[0],
                        #[rustfmt::skip]
                        9  => next_value = Gsharp.next()[0],
                        10 => next_value = A.next()[0],
                        11 => next_value = Asharp.next()[0],
                        12 => next_value = B.next()[0],
                        #[rustfmt::skip]
                        _  => next_value = 0.0,
                    }
                    *elem = (next_value / 5.0) as f32;
                }
            }
            _ => (),
        }
    });
}
