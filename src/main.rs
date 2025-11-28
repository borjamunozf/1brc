use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    process::exit,
};

struct WeatherStation {
    min_measurement: f32,
    max_measurement: f32,
    avg_measurement: f32,
    total_measurements: f32,
}

fn main() {
    let mut weather_stations: HashMap<String, WeatherStation> = HashMap::new();

    let args: Vec<String> = env::args().collect();
    let f_measurements = &args[1];

    let file = File::open(f_measurements).unwrap_or_else(|err| {
        eprintln!("Could not open file: {err}");
        exit(-1)
    });

    let f = BufReader::new(file);
    for line in f.lines() {
        let line = line.unwrap();
        let splitted: Vec<&str> = line.split(";").collect();
        let measurement = splitted[1].parse::<f32>().unwrap();
        match weather_stations.get_mut(splitted[0]) {
            Some(v) => {
                v.total_measurements += 1.0;
                if measurement < v.min_measurement {
                    v.min_measurement = measurement;
                }

                if measurement > v.max_measurement {
                    v.max_measurement = measurement
                }

                v.avg_measurement = (measurement - v.avg_measurement) / v.total_measurements
            }
            None => {
                weather_stations.insert(
                    splitted[0].to_owned(),
                    WeatherStation {
                        min_measurement: measurement,
                        max_measurement: measurement,
                        avg_measurement: measurement,
                        total_measurements: 1.0,
                    },
                );
            }
        }
    }

    let mut sorted: Vec<_> = weather_stations.iter().collect();
    sorted.sort_by_key(|k| k.0);

    print!("{{");

    for (i, (key, val)) in sorted.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }

        print!(
            "{}={:.1}/{:.1}/{:.1}",
            key, val.min_measurement, val.avg_measurement, val.max_measurement
        );
    }

    println!("}}");
}
