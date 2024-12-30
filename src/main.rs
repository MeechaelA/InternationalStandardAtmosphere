use std::fs::File;
use std::io::prelude::*;
use icao_standard_atmosphere::isa;
fn main() {
   
    let mut icao_standard_atmosphere = isa::ISA::new();
    let mut file = File::create("output_file.csv").unwrap();

    let altitudes = -5..81;
    file.write_all(b"altitude,temperature,pressure,density\n");
    for value in altitudes{
        let test_altitude = 1000.0*f64::from(value);
        let temperature = icao_standard_atmosphere.temperature(test_altitude).expect("Layer not found.");
        let pressure = icao_standard_atmosphere.pressure(test_altitude).expect("Layer not found.");
        let density = icao_standard_atmosphere.density(test_altitude).expect("Layer not found.");

        file.write_fmt(format_args!("{},{},{},{}\n",test_altitude,temperature,pressure,density)).unwrap();
    }
}
