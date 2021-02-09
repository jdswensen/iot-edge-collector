use clap::{App, Arg};
use futures::prelude::*;
use influxdb2_client;
use sensehat::{SenseHat};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct EndpointConfig {
    org: String,
    bucket: String,
    url: String,
    token: String,
}

impl Default for EndpointConfig {
    fn default() -> Self {
        EndpointConfig {
            org: "myorg".to_string(),
            bucket: "mybucket".to_string(),
            url: "http://localhost:9999".to_string(),
            token: "my-token".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Aggregate sensor data and beam it to the cloud.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Path to endpoint configuration file (JSON)")
                .takes_value(true)
        )
        .get_matches();

    let endpoint = match cli_matches.value_of("config") {
        Some(fp) => {
            let config_file = File::open(&fp)?;
            let reader = BufReader::new(config_file);
            serde_json::from_reader(reader)?
        }
        None => {
            EndpointConfig::default()
        }
    };


    let client = influxdb2_client::Client::new(endpoint.url.as_str(), endpoint.token.as_str());
    let mut sensehat = SenseHat::new().unwrap();

    loop {
        let points = sample_temperature(sensehat.borrow_mut())?;
        client.write(endpoint.org.as_str(), endpoint.bucket.as_str(), stream::iter(points)).await?;
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

fn sample_temperature(sensehat: &mut SenseHat) -> Result<Vec<influxdb2_client::DataPoint>, Box<dyn std::error::Error>> {
    // Real world sensor reading wouldn't necessarily grab raw values like this. Temperature
    // data doesn't flucuate as much as IMU data might, but a better approach would be to
    // oversample the data before submitting.
    let temperature_humidity = sensehat.get_temperature_from_humidity().unwrap();
    let temperature_humidity_datapoint =
        influxdb2_client::DataPoint::builder("temperature")
            .tag("host", "dev")
            .tag("source", "humidity")
            .field("value", temperature_humidity.as_celsius())
            .build()?;

    let temperature_pressure = sensehat.get_temperature_from_pressure().unwrap();
    let temperature_pressure_datapoint =
        influxdb2_client::DataPoint::builder("temperature")
            .tag("host", "dev")
            .tag("source", "pressure")
            .field("value", temperature_pressure.as_celsius())
            .build()?;

    Ok(vec![
        temperature_humidity_datapoint,
        temperature_pressure_datapoint
    ])
}
