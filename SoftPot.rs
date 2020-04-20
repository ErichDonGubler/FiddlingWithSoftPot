use serial::prelude::*;
use sample::{signal, Signal};

use std::io::Read;
use std::thread;
use std::sync::{Mutex, Arc};

// use rand::Rng;
use cpal::{StreamData, UnknownTypeOutputBuffer};
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

fn main() {
    // Set up port to get data from arduino
    let mut port = serial::open("COM4").unwrap();
    port.reconfigure(&|settings|{
        settings.set_baud_rate(serial::Baud9600)
    }).unwrap();

    // Set up audio stuff
    let host = cpal::default_host();
    let event_loop = host.event_loop();
    let device = host.default_output_device().expect("no output device available");
    let mut supported_formats_range = device.supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range.next()
        .expect("no supported format?!")
        .with_max_sample_rate();    
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id).expect("failed to play_stream");

    // Define each note
    let mut C      = signal::rate(44_100.0).const_hz(261.626).sine();
    let mut Csharp = signal::rate(44_100.0).const_hz(277.183).sine();
    let mut D      = signal::rate(44_100.0).const_hz(293.665).sine();
    let mut Dsharp = signal::rate(44_100.0).const_hz(311.127).sine();
    let mut E      = signal::rate(44_100.0).const_hz(329.628).sine();
    let mut F      = signal::rate(44_100.0).const_hz(349.228).sine();
    let mut Fsharp = signal::rate(44_100.0).const_hz(369.994).sine();
    let mut G      = signal::rate(44_100.0).const_hz(391.995).sine();
    let mut Gsharp = signal::rate(44_100.0).const_hz(415.305).sine();
    let mut A      = signal::rate(44_100.0).const_hz(440.000).sine();
    let mut Asharp = signal::rate(44_100.0).const_hz(466.164).sine();
    let mut B      = signal::rate(44_100.0).const_hz(493.883).sine();

    let current_note = Arc::new(Mutex::new(0));
    let get_note = Arc::clone(&current_note);
    let play_note = Arc::clone(&current_note);
    thread::spawn(move || {
        loop {
            let mut slice = [0;2];
            let resultythingy = port.read_exact(&mut slice[..]);
            if resultythingy.is_ok() {
                // println!("                                      Everything is ok!");
                // The arduino sends ascii, and 0 in ascii is 48.
                let read_note = (i32::from(slice[0]%48)* 10) + i32::from(slice[1]%48);
                let mut note = get_note.lock().unwrap();
                if read_note != *note {
                    *note = read_note
                }
                // println!("                                          Note! {:?}", *note)
            } else if resultythingy.is_err() {
                // println!("                                      Oh no! {:?}", resultythingy);
            }
        }
    });
    event_loop.run(move |_stream_id, _stream_result| {
        let note = play_note.lock().unwrap();
        println!("reading in event note: {}", *note);
        let stream_data = _stream_result.unwrap();
        match stream_data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                // I was fiddling with static as a test before I figured out specific notes.
                // let mut rng = rand::thread_rng();
                for elem in buffer.iter_mut() {
                    // *elem = rng.gen::<f32>()
                    match *note {
                        0 => *elem = 0.0,
                        1 => *elem = C.next()[0] as f32,
                        2 => *elem = Csharp.next()[0] as f32,
                        3  => *elem = D.next()[0] as f32,
                        4 => *elem = Dsharp.next()[0] as f32,
                        5 => *elem = E.next()[0] as f32,
                        6 => *elem = F.next()[0] as f32,
                        7 => *elem = Fsharp.next()[0] as f32,
                        8 => *elem = G.next()[0] as f32,
                        9 => *elem = Gsharp.next()[0] as f32,
                        10 => *elem = A.next()[0] as f32,
                        11 => *elem = Asharp.next()[0] as f32,
                        12 => *elem = B.next()[0] as f32,
                        _ => *elem = 0.0
                    }
                }
            },
            _ => ()
        }
    });
}