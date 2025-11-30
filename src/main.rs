use core::f32;
use std::{
    collections::HashMap, env, fs::{File}, io::{BufRead, BufReader}, process::exit
};

struct WeatherStation {
    min_measurement: f32,
    max_measurement: f32,
    avg_measurement: f32,
    total_measurements: f32,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut weather_stations: HashMap<String, WeatherStation> = HashMap::new();

    let args: Vec<String> = env::args().collect();
    let f_measurements = &args[1];

    let file = File::open(f_measurements).unwrap_or_else(|err| {
        eprintln!("Could not open file: {err}");
        exit(-1)
    });

    let mut f = BufReader::new(file);
   
    let mut buf: Vec<u8> = Vec::new();
    while f.read_until(b'\n', &mut buf)? != 0 {
        let sep = buf.iter().position(|&b| b == b';').unwrap();
        let station_bytes = &buf[..sep];
        let meas_bytes  = &buf[sep + 1..];

        let station = unsafe {std::str::from_utf8_unchecked(station_bytes)};
        let measurement: f32 = parse_f32(meas_bytes);
        match weather_stations.get_mut(station) {
            Some(v) => {
                v.total_measurements += 1.0;
                if measurement < v.min_measurement {
                    v.min_measurement = measurement;
                }

                if measurement > v.max_measurement {
                    v.max_measurement = measurement
                }

                v.avg_measurement += (measurement - v.avg_measurement) / v.total_measurements
            }
            None => {
                weather_stations.insert(
                    station.to_owned(),
                    WeatherStation {
                        min_measurement: measurement,
                        max_measurement: measurement,
                        avg_measurement: measurement,
                        total_measurements: 1.0,
                    },
                );
            }
        }
        buf.clear();
    }

    let mut sorted: Vec<_> = weather_stations.iter().collect();
    sorted.sort_by_key(|k| k.0);

    print!("{{");

    for (i, (key, val)) in sorted.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }

        print!(
            "{}={:?}/{:?}/{:?}",
            key, val.min_measurement, val.max_measurement, val.avg_measurement.round()
        );
    }

    println!("}}");
    Ok(())
}

fn parse_f32(bytes: &[u8]) -> f32 {
    let mut i = 0;
    let mut sign = 1.0;

    if bytes[i] == b'-' {
        sign = -1.0;
        i += 1;
    }

    let mut int_part = (bytes[i] - b'0') as f32;
    i += 1;

    if bytes[i] != b'.' {
        int_part = int_part * 10.0 + (bytes[i] - b'0') as f32;
        i += 1;
    }

    i += 1;

    let frac = (bytes[i] - b'0') as f32 * 0.1;

    sign * (int_part + frac)
}
