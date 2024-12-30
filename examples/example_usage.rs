use icao_standard_atmosphere::isa;

fn main() {
    let icao_standard_atmosphere = isa::ISA::new();
    let test_altitude = 0.0;
    let temperature = icao_standard_atmosphere.temperature(test_altitude).expect("Layer not found.");
    let pressure = icao_standard_atmosphere.pressure(test_altitude).expect("Layer not found.");
    let density = icao_standard_atmosphere.density(test_altitude).expect("Layer not found.");
    println!("Temperature: {}", temperature);
    println!("Pressure: {}", pressure);
    println!("Density: {}", density);
}
