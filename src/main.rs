use std::{io::{Error, ErrorKind}, time::{SystemTime, Duration}, ops::Sub, thread, sync::RwLock};
use clap::Parser;
use evdev::{Device, Key, EventType, InputEventKind};
use pad::{PadStr, Alignment};

const WORD_LENGTH: usize = 5;

static LOG: RwLock<Vec<SystemTime>> = RwLock::new(vec![]);

/// A simple tool to track your typing speed. Designed to be used in conjunction with waybar
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None
)]
struct CliArgs {
    /// Count CPM instead of WPM
    #[arg(short, long)]
    cpm: bool,

    /// Show best score
    #[arg(short, long)]
    best: bool,

    /// Pad values with leeding 0's
    #[arg(short, long, default_value="2")]
    pad: usize,

    /// Set the interval at wich readings are taken (in seconds)
    #[arg(short, long, default_value="5")]
    interval: u64,

    /// manually set the keyboard device to track
    #[arg()]
    device: String
}


fn main() -> Result<(), Error> {
    let args = CliArgs::parse();

    let device = Device::open(args.device.as_str())?;
    if !device.supported_keys().map_or(false, |keys| keys.contains(Key::KEY_ENTER)) {
        return Err(Error::new(ErrorKind::Other, "not a keyboard"));
    }

    event_loop(device);
    read_loop(args);

    Ok(())
}

fn read_loop(args: CliArgs) {
    let mut best = 0;
    loop {
        thread::sleep(Duration::from_secs(args.interval));

        let threshold = SystemTime::now().sub(Duration::from_secs(60));
        let mut count: usize;

        {
            let mut keypresses = LOG.write().unwrap();
            keypresses.retain(|x| x >= &threshold);
            count = keypresses.len();
        }

        if args.cpm {
            count /= WORD_LENGTH;
        }

        if count > best {
            best = count;
        }

        if args.best {
            println!(
                "{}/{}", 
                count.to_string().pad(args.pad, '0', Alignment::Right, false),
                best.to_string().pad(args.pad, '0', Alignment::Right, false)
            );
        } else {
            println!(
                "{}", 
                count.to_string().pad(args.pad, '0', Alignment::Right, false)
            );
        }
    }
}

fn event_loop(mut device: Device) {
    thread::spawn(move || {
        loop {
            for ev in device.fetch_events().unwrap() {
                if ev.event_type() != EventType::KEY || ev.value() != 0 {
                    continue;
                }
                if ev.kind() == InputEventKind::Key(Key::KEY_BACKSPACE) {
                    continue;
                }

                let mut keypresses = LOG.write().unwrap();
                keypresses.push(SystemTime::now());
            }
        }
    });
}
