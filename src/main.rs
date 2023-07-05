use battery::units::power::watt;
use battery::units::ratio::percent;
use battery::Manager;
use battery::State;
use chrono::Local;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() -> battery::Result<()> {
    let wait_time: u64 = match env::args().nth(1) {
        Some(arg1) => arg1
            .parse()
            .expect("Not a valid number of seconds passed as an argument"),
        None => 1800,
    };

    let logs_path: String = format!(
        "{}/{}",
        home::home_dir().unwrap().display(),
        ".cache/batt_stats"
    );

    if !Path::new(logs_path.as_str()).exists() {
        fs::create_dir_all(logs_path.as_str()).expect("Failed to create cache directory");
    }

    let (mut battery, manager) = get_battery().expect("Failed to get battery!");

    let mut log_file: Box<dyn Write> = Box::new(io::stdout());
    let mut reset_file = true;

    // Infinite loop to detect the current state of the battery
    loop {
        // If the battery is discharging and it just came from a different state, it creates a new log file
        if battery.state() == State::Discharging {
            if reset_file {
                log_file = Box::new(manage_log_file());
                reset_file = false;
            }
            let current_percent = battery.state_of_charge().get::<percent>();
            let current_rate = battery.energy_rate().get::<watt>();

            writeln!(
                log_file,
                "{} {:?} {:?}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                current_percent,
                current_rate,
            )
            .expect("Failed to write to the log file.");
        } else {
            // If the state is Charging, create a new file next time the state is Discharge
            reset_file = true;
        }

        sleep(Duration::from_secs(wait_time));
        manager.refresh(&mut battery)?;
    }
}

fn get_battery() -> battery::Result<(battery::Battery, battery::Manager)> {
    let manager = Manager::new()?;

    match manager.batteries()?.next() {
        Some(Ok(battery)) => Ok((battery, manager)),
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            Err(e.into())
        }
        None => {
            eprintln!("Unable to find any batteries");
            Err(io::Error::new(io::ErrorKind::NotFound, "No batteries found").into())
        }
    }
}

fn manage_log_file() -> std::fs::File {
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_file_path: String = format!(
        "{}/{}/{}.txt",
        home::home_dir().unwrap().display(),
        ".cache/batt_stats",
        current_time
    );

    let log_file_path = Path::new(&log_file_path);

    // Create or open the log file
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Failed to open the log file.")
}
