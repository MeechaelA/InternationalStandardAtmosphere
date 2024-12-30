mod layers;
mod reference;

pub mod isa{
    use std::fmt::Error;
    use crate::layers::*;
    use crate::reference::constants::*;

    #[derive(Debug)]
    pub enum Constraint{
        NoGradient
    }

    pub struct ISA{
        base_geopotential_altitudes: [f64; 9],
        base_geopotential_densities: [f64; 9],
        base_temperatures: [f64; 9],
        gradients: [f64; 8]
    }
    
    impl ISA{
        pub fn new()->Self{
            ISA{
                base_geopotential_altitudes: [-5000.0,   0.0,        11000.0,        20000.0,    32000.0,    47000.0,    51000.0,    71000.0,    80000.0],
                base_geopotential_densities: [1.93047,   1.225,      3.63918e-1,    8.80345e-2, 1.32249e-2,  1.42752e-3, 8.616e-4,   6.42105e-5, 1.57004e-5],
                base_temperatures:           [320.65,    288.15,     216.65,         216.65,     228.65,     270.65,     270.65,     214.650,    196.65],
                gradients:                   [-6.5e-3,   -6.5e-3,    0.0,            1.0e-3,     2.8e-3,     0.0,        -2.8e-3,    -2.0e-3]
            }
        }
        pub fn get_base_index(&self, geopotential_altitude: f64)->Result<usize, &'static str>{
            let mut i = 0;
            for value in self.base_geopotential_altitudes{
                if geopotential_altitude <= value{
                    return Ok(i);
                }
                else{
                    i+=1;
                }
            }
            return Err("yikes...");
        }
        pub fn get_gradient_index(&self, base_index: usize)->Result<usize, Constraint>{
            if base_index == self.base_geopotential_altitudes.len(){
                return Err(Constraint::NoGradient);
            }
            if base_index == 0{
                return Ok(base_index);
            }
            else{
                return Ok(base_index-1);
            }
        }

        pub fn gravity(geometric_altitude: f64)->f64{
            g_0 * (earth_radius_nominal / (earth_radius_nominal + geometric_altitude)).powf(2.0)
        }
        pub fn geopotential_altitude(&self, geometric_altitude: f64)->f64{
            (earth_radius_nominal*geometric_altitude)/(earth_radius_nominal+geometric_altitude)
        }
        pub fn geometric_altitude(&self, geopotential_altitude: f64)->f64{
            (earth_radius_nominal*geopotential_altitude)/(earth_radius_nominal-geopotential_altitude)
        }
        
        fn base_temperature(&self, geopotential_altitude: f64)->Result<f64, &'static str>{
            let base_index = self.get_base_index(geopotential_altitude)?;
            return Ok(self.base_temperatures[base_index]);
        }

        fn base_density(&self, geopotential_altitude: f64)->Result<f64, &'static str>{
            let base_index = self.get_base_index(geopotential_altitude)?;
            return Ok(self.base_geopotential_densities[base_index]);
        }

        fn base_pressure(&self, geopotential_altitude: f64)->Result<f64, &'static str>{
            let base_density = self.base_density(geopotential_altitude)?;
            let base_temperature = self.base_temperature(geopotential_altitude)?;
            let base_pressure = base_density * (R_Star / air_molar_mass) * base_temperature;
            return Ok(base_pressure);
        }

        pub fn temperature(&self, geopotential_altitude: f64)->Result<f64, &'static str>{
            let base_index = self.get_base_index(geopotential_altitude)?;
            let gradient_index_result = self.get_gradient_index(base_index);
            match gradient_index_result{
                Ok(gradient_index)=>{
                    return Ok(self.base_temperatures[base_index] + self.gradients[gradient_index]*(geopotential_altitude - self.base_geopotential_altitudes[base_index]));
                },
                Err(Constraint::NoGradient)=>{
                    // Not sure this is right...
                    return Ok(self.base_temperatures[base_index]);
                }
            }
        }

        pub fn pressure(&self, geopotential_altitude: f64)->Result<f64, &'static str>{
            let base_index = self.get_base_index(geopotential_altitude)?;
            let gradient_index_result = self.get_gradient_index(base_index);

            match gradient_index_result{
                Ok(gradient_index)=>{
                    let base_temperature = self.base_temperatures[base_index];
                    let base_pressure = self.base_pressure(geopotential_altitude)?;
                    let temperature = self.temperature(geopotential_altitude)?;
                    let gradient = self.gradients[gradient_index];

                    if gradient != 0.0{
                        let t1 = base_pressure;
                        let t2 = 1.0+(gradient/base_temperature)*(geopotential_altitude - self.base_geopotential_altitudes[base_index]);
                        let t3 = -g_0 / (R*gradient);
                        let pressure = t1 * t2.powf(t3);
                        return Ok(pressure)
                    }
                    else{
                        let pressure = base_pressure * ((-g_0 / (R * temperature)) * (geopotential_altitude - self.base_geopotential_altitudes[base_index])).exp();
                        return Ok(pressure)
                    }
                },
                Err(Constraint::NoGradient)=>{
                    return Err("No Gradient")
                }
            }
        }

        pub fn density(&self, geopotential_altitude: f64)->Result<f64, &'static str>{
            let temperature = self.temperature(geopotential_altitude)?;
            let pressure = self.pressure(geopotential_altitude)?;
            let density = pressure / ((R_Star / air_molar_mass) * temperature);
            return Ok(density);
        }
    }
}
