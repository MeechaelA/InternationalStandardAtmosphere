// 2.1 Primary Constants - Table A - Primary pub constants and characteristics adopted for the calculation of the ICAO standard atmosphere
pub const g_0:       f64 = 9.80665;      // m/s2
pub const M_0:       f64 = 28.964420;    // kg/kmol
pub const N_A:       f64 = 602.257e24;   // kmol^{-1}
pub const P_0:       f64 = 101.325e3;    // Pa
pub const R_Star:    f64 = 8314.32;      // kg*m2/(s2*K*kmol)
pub const R:         f64 = 287.05287;    // m2/(K*s2)
pub const S:         f64 = 110.4;        // K
pub const T_I:       f64 = 273.15;       // K
pub const T_0:       f64 = 288.15;       // K
pub const t_i:       f64 = 0.0;          // deg C   
pub const t_0:       f64 = 15.0;         // deg C
pub const Beta_s:    f64 = 1.458e-6;     // kg/(m*s*K12)
pub const k:         f64 = 1.4;          // dimensionless
pub const rho_0:     f64 = 1.225;        // kg/m3
pub const sigma:     f64 = 0.365e-9;     // m
// Other Constants
pub const earth_radius_nominal: f64 = 6356766.0;    // m
// 2.1 Primary Constants - Table B - Dry, clean air composition near sea level1
// Content of Volume (%)
pub const nitrogen_volume: f64 = 78.084;
pub const oxygen_volume: f64 = 20.9476;
pub const argon_volume: f64 = 0.934;
pub const carbon_dioxide_volume: f64 = 0.0314;
pub const neon_volume: f64 = 1.818e-3;
pub const helium_volume: f64 = 524.0e-6;
pub const krypton_volume: f64 = 114.0e-6;
pub const xenon_volume: f64 = 8.7e-6;
pub const hydrogen_volume: f64 = 50.0e-6;
pub const nitrogen_monoxide_volume: f64 = 50.0e-6;
pub const methane_volume: f64 = 0.2e-3;
pub const ozone_summer_volume: f64 = 7.0e-6;
pub const ozone_winter_volume: f64 = 2.0e-6;
pub const sulphur_dioxide_volume: f64 = 0.1e-3;
pub const nitrogen_dioxide_volume: f64 = 2.0e-6;
pub const iodine_volume: f64 = 1.0e-6;
pub const air_volume: f64 = 100.0;
// Molar Mass M (kg/kmol)
pub const nitrogen_molar_mass: f64 = 28.0134;
pub const oxygen_molar_mass: f64 = 31.9988;
pub const argon_molar_mass: f64 = 39.948;
pub const neon_molar_mass: f64 = 20.183;
pub const carbon_dioxide_molar_mass: f64 = 44.00995;
pub const helium_molar_mass: f64 = 4.0026;
pub const krypton_molar_mass: f64 = 83.80;
pub const xenon_molar_mass: f64 = 131.30;
pub const hydrogen_molar_mass: f64 = 2.01594;
pub const nitrogen_monoxide_molar_mass: f64 = 44.0128;
pub const methane_molar_mass: f64 = 16.04303;
pub const ozone_summer_molar_mass: f64 = 47.9982;
pub const ozone_winter_molar_mass: f64 = 47.9982;
pub const sulphur_dioxide_molar_mass: f64 = 64.0628;
pub const nitrogen_dioxide_molar_mass: f64 = 46.0055;
pub const iodine_molar_mass: f64 = 253.8088;
pub const air_molar_mass: f64 = 28.964420;
// Table C Physical Characteristics of the Atmosphere at mean sea level
pub const a_0: f64 = 340.294;
pub const H_p_0: f64 = 8434.5;
pub const l_0: f64 = 66.328e-9;
pub const n_0: f64 = 25.471e24;
pub const v_0: f64 = 458.94;
pub const gamma_0: f64 = 12.013;
pub const nu_0: f64 = 14.607e-6;
pub const lambda_0: f64 = 25.343e-3;
pub const mu_0: f64 = 17.894e-6;
pub const omega_0: f64 = 6.9193e9;