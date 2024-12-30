use std::fs::File;
use std::io::prelude::*;
use international_standard_atmosphere::isa;

#[test]
fn build_temperature_pressure_density_table() {
    let geometric_altitudes = [
        -5000.0,-4950.0,-4900.0,-4850.0,-4800.0,-4750.0,-4700.0,-4650.0,-4600.0,-4550.0,-4500.0,-4450.0,-4400.0,-4350.0,-4300.0,-4250.0,-4200.0,-4150.0,-4100.0,-4050.0,-4000.0,-3950.0,-3900.0,-3850.0,-3800.0,-3750.0,-3700.0,-3650.0,-3600.0,-3550.0,-3500.0,-3450.0,-3400.0,-3350.0,-3300.0,-3250.0,-3200.0,-3150.0,-3100.0,-3050.0,-3000.0,-2950.0,-2900.0,-2850.0,-2800.0,-2750.0,-2700.0,-2650.0,-2600.0,-2550.0,-2500.0,-2450.0,-2400.0,-2350.0,-2300.0,-2250.0,-2200.0,-2150.0,-2100.0,-2050.0,-2000.0,-1950.0,-1900.0,-1850.0,-1800.0,-1750.0,-1700.0,-1650.0,-1600.0,-1550.0,-1500.0,-1450.0,-1400.0,-1350.0,-1300.0,-1250.0,-1200.0,-1150.0,-1100.0,-1050.0,-1000.0,-950.0,-900.0,-850.0,-800.0,-750.0,-700.0,-650.0,-600.0,-550.0,-500.0,-450.0,-400.0,-350.0,-300.0,-250.0,-200.0,-150.0,-100.0,-50.0,0.0,50.0,100.0,150.0,200.0,250.0,300.0,350.0,400.0,450.0,500.0,550.0,600.0,650.0,700.0,750.0,800.0,850.0,900.0,950.0,1000.0,1050.0,1100.0,1150.0,1200.0,1250.0,1300.0,1350.0,1400.0,1450.0,1500.0,1550.0,1600.0,1650.0,1700.0,1750.0,1800.0,1850.0,1900.0,1950.0,2000.0,2050.0,2100.0,2150.0,2200.0,2250.0,2300.0,2350.0,2400.0,2450.0,2500.0,2550.0,2600.0,2650.0,2700.0,2750.0,2800.0,2850.0,2900.0,2950.0,3000.0,3050.0,3100.0,3150.0,3200.0,3250.0,3300.0,3350.0,3400.0,3450.0,3500.0,3550.0,3600.0,3650.0,3700.0,3750.0,3800.0,3850.0,3900.0,3950.0,4000.0,4050.0,4100.0,4150.0,4200.0,4250.0,4300.0,4350.0,4400.0,4450.0,4500.0,4550.0,4600.0,4650.0,4700.0,4750.0,4800.0,4850.0,4900.0,4950.0,5000.0,5050.0,5100.0,5150.0,5200.0,5250.0,5300.0,5350.0,5400.0,5450.0,5500.0,5550.0,5600.0,5650.0,5700.0,5750.0,5800.0,5850.0,5900.0,5950.0,6000.0,6050.0,6100.0,6150.0,6200.0,6250.0,6300.0,6350.0,6400.0,6450.0,6500.0,6550.0,6600.0,6650.0,6700.0,6750.0,6800.0,6850.0,6900.0,6950.0,7000.0,7050.0,7100.0,7150.0,7200.0,7250.0,7300.0,7350.0,7400.0,7450.0,7500.0,7550.0,7600.0,7650.0,7700.0,7750.0,7800.0,7850.0,7900.0,7950.0,8000.0,8050.0,8100.0,8150.0,8200.0,8250.0,8300.0,8350.0,8400.0,8450.0,8500.0,8550.0,8600.0,8650.0,8700.0,8750.0,8800.0,8850.0,8900.0,8950.0,9000.0,9050.0,9100.0,9150.0,9200.0,9250.0,9300.0,9350.0,9400.0,9450.0,9500.0,9550.0,9600.0,9650.0,9700.0,9750.0,9800.0,9850.0,9900.0,9950.0,10000.0,10050.0,10100.0,10150.0,10200.0,10250.0,10300.0,10350.0,10400.0,10450.0,10500.0,10550.0,10600.0,10650.0,10700.0,10750.0,10800.0,10850.0,10900.0,10950.0,11000.0,11050.0,11100.0,11150.0,11200.0,11250.0,11300.0,11350.0,11400.0,11450.0,11500.0,11550.0,11600.0,11650.0,11700.0,11750.0,11800.0,11850.0,11900.0,11950.0,12000.0,12050.0,12100.0,12150.0,12200.0,12250.0,12300.0,12350.0,12400.0,12450.0,12500.0,12550.0,12600.0,12650.0,12700.0,12750.0,12800.0,12850.0,12900.0,12950.0,13000.0,13050.0,13100.0,13150.0,13200.0,13250.0,13300.0,13350.0,13400.0,13450.0,13500.0,13550.0,13600.0,13650.0,13700.0,13750.0,13800.0,13850.0,13900.0,13950.0,14000.0,14050.0,14100.0,14150.0,14200.0,14250.0,14300.0,14350.0,14400.0,14450.0,14500.0,14550.0,14600.0,14650.0,14700.0,14750.0,14800.0,14850.0,14900.0,14950.0,15000.0,15050.0,15100.0,15150.0,15200.0,15250.0,15300.0,15350.0,15400.0,15450.0,15500.0,15550.0,15600.0,15650.0,15700.0,15750.0,15800.0,15850.0,15900.0,15950.0,16000.0,16050.0,16100.0,16150.0,16200.0,16250.0,16300.0,16350.0,16400.0,16450.0,16500.0,16550.0,16600.0,16650.0,16700.0,16750.0,16800.0,16850.0,16900.0,16950.0,17000.0,17050.0,17100.0,17150.0,17200.0,17250.0,17300.0,17350.0,17400.0,17450.0,17500.0,17550.0,17600.0,17650.0,17700.0,17750.0,17800.0,17850.0,17900.0,17950.0,18000.0,18050.0,18100.0,18150.0,18200.0,18250.0,18300.0,18350.0,18400.0,18450.0,18500.0,18550.0,18600.0,18650.0,18700.0,18750.0,18800.0,18850.0,18900.0,18950.0,19000.0,19050.0,19100.0,19150.0,19200.0,19250.0,19300.0,19350.0,19400.0,19450.0,19500.0,19550.0,19600.0,19650.0,19700.0,19750.0,19800.0,19850.0,19900.0,19950.0,20000.0,20050.0,20100.0,20150.0,20200.0,20250.0,20300.0,20350.0,20400.0,20450.0,20500.0,20550.0,20600.0,20650.0,20700.0,20750.0,20800.0,20850.0,20900.0,20950.0,21000.0,21050.0,21100.0,21150.0,21200.0,21250.0,21300.0,21350.0,21400.0,21450.0,21500.0,21550.0,21600.0,21650.0,21700.0,21750.0,21800.0,21850.0,21900.0,21950.0,22000.0,22050.0,22100.0,22150.0,22200.0,22250.0,22300.0,22350.0,22400.0,22450.0,22500.0,22550.0,22600.0,22650.0,22700.0,22750.0,22800.0,22850.0,22900.0,22950.0,23000.0,23050.0,23100.0,23150.0,23200.0,23250.0,23300.0,23350.0,23400.0,23450.0,23500.0,23550.0,23600.0,23650.0,23700.0,23750.0,23800.0,23850.0,23900.0,23950.0,24000.0,24050.0,24100.0,24150.0,24200.0,24250.0,24300.0,24350.0,24400.0,24450.0,24500.0,24550.0,24600.0,24650.0,24700.0,24750.0,24800.0,24850.0,24900.0,24950.0,25000.0,25050.0,25100.0,25150.0,25200.0,25250.0,25300.0,25350.0,25400.0,25450.0,25500.0,25550.0,25600.0,25650.0,25700.0,25750.0,25800.0,25850.0,25900.0,25950.0,26000.0,26050.0,26100.0,26150.0,26200.0,26250.0,26300.0,26350.0,26400.0,26450.0,26500.0,26550.0,26600.0,26650.0,26700.0,26750.0,26800.0,26850.0,26900.0,26950.0,27000.0,27050.0,27100.0,27150.0,27200.0,27250.0,27300.0,27350.0,27400.0,27450.0,27500.0,27550.0,27600.0,27650.0,27700.0,27750.0,27800.0,27850.0,27900.0,27950.0,28000.0,28050.0,28100.0,28150.0,28200.0,28250.0,28300.0,28350.0,28400.0,28450.0,28500.0,28550.0,28600.0,28650.0,28700.0,28750.0,28800.0,28850.0,28900.0,28950.0,29000.0,29050.0,29100.0,29150.0,29200.0,29250.0,29300.0,29350.0,29400.0,29450.0,29500.0,29550.0,29600.0,29650.0,29700.0,29750.0,29800.0,29850.0,29900.0,29950.0,30000.0,30050.0,30100.0,30150.0,30200.0,30250.0,30300.0,30350.0,30400.0,30450.0,30500.0,30550.0,30600.0,30650.0,30700.0,30750.0,30800.0,30850.0,30900.0,30950.0,31000.0,31050.0,31100.0,31150.0,31200.0,31250.0,31300.0,31350.0,31400.0,31450.0,31500.0,31550.0,31600.0,31650.0,31700.0,31750.0,31800.0,31850.0,31900.0,31950.0,32000.0,32100.0,32200.0,32300.0,32400.0,32500.0,32600.0,32700.0,32800.0,32900.0,33000.0,33100.0,33200.0,33300.0,33400.0,33500.0,33600.0,33700.0,33800.0,33900.0,34000.0,34100.0,34200.0,34300.0,34400.0,34500.0,34600.0,34700.0,34800.0,34900.0,35000.0,35100.0,35200.0,35300.0,35400.0,35500.0,35600.0,35700.0,35800.0,35900.0,36000.0,36100.0,36200.0,36300.0,36400.0,36500.0,36600.0,36700.0,36800.0,36900.0,37000.0,37100.0,37200.0,37300.0,37400.0,37500.0,37600.0,37700.0,37800.0,37900.0,38000.0,38100.0,38200.0,38300.0,38400.0,38500.0,38600.0,38700.0,38800.0,38900.0,39000.0,39100.0,39200.0,39300.0,39400.0,39500.0,39600.0,39700.0,39800.0,39900.0,40000.0,40100.0,40200.0,40300.0,40400.0,40500.0,40600.0,40700.0,40800.0,40900.0,41000.0,41100.0,41200.0,41300.0,41400.0,41500.0,41600.0,41700.0,41800.0,41900.0,42000.0,42100.0,42200.0,42300.0,42400.0,42500.0,42600.0,42700.0,42800.0,42900.0,43000.0,43100.0,43200.0,43300.0,43400.0,43500.0,43600.0,43700.0,43800.0,43900.0,44000.0,44100.0,44200.0,44300.0,44400.0,44500.0,44600.0,44700.0,44800.0,44900.0,45000.0,45100.0,45200.0,45300.0,45400.0,45500.0,45600.0,45700.0,45800.0,45900.0,46000.0,46100.0,46200.0,46300.0,46400.0,46500.0,46600.0,46700.0,46800.0,46900.0,47000.0,47100.0,47200.0,47300.0,47400.0,47500.0,47600.0,47700.0,47800.0,47900.0,48000.0,48100.0,48200.0,48300.0,48400.0,48500.0,48600.0,48700.0,48800.0,48900.0,49000.0,49100.0,49200.0,49300.0,49400.0,49500.0,49600.0,49700.0,49800.0,49900.0,50000.0,50100.0,50200.0,50300.0,50400.0,50500.0,50600.0,50700.0,50800.0,50900.0,51000.0,51200.0,51400.0,51600.0,51800.0,52000.0,52200.0,52400.0,52600.0,52800.0,53000.0,53200.0,53400.0,53600.0,53800.0,54000.0,54200.0,54400.0,54600.0,54800.0,55000.0,55200.0,55400.0,55600.0,55800.0,56000.0,56200.0,56400.0,56600.0,56800.0,57000.0,57200.0,57400.0,57600.0,57800.0,58000.0,58200.0,58400.0,58600.0,58800.0,59000.0,59200.0,59400.0,59600.0,59800.0,60000.0,60200.0,60400.0,60600.0,60800.0,61000.0,61200.0,61400.0,61600.0,61800.0,62000.0,62200.0,62400.0,62600.0,62800.0,63000.0,63200.0,63400.0,63600.0,63800.0,64000.0,64200.0,64400.0,64600.0,64800.0,65000.0,65200.0,65400.0,65600.0,65800.0,66000.0,66200.0,66400.0,66600.0,66800.0,67000.0,67200.0,67400.0,67600.0,67800.0,68000.0,68200.0,68400.0,68600.0,68800.0,69000.0,69200.0,69400.0,69600.0,69800.0,70000.0,70200.0,70400.0,70600.0,70800.0,71000.0,71200.0,71400.0,71600.0,71800.0,72000.0,72200.0,72400.0,72600.0,72800.0,73000.0,73200.0,73400.0,73600.0,73800.0,74000.0,74200.0,74400.0,74600.0,74800.0,75000.0,75200.0,75400.0,75600.0,75800.0,76000.0,76200.0,76400.0,76600.0,76800.0,77000.0,77200.0,77400.0,77600.0,77800.0,78000.0,78200.0,78400.0,78600.0,78800.0,79000.0,79200.0,79400.0,79600.0,79800.0,80000.0
    ];

    let mut icao_standard_atmosphere = isa::ISA::new();
    let mut file = File::create("output_file.csv").unwrap();

    file.write_all(b"altitude,temperature,pressure,density\n");
    for geometric_altitude in geometric_altitudes{
        let geopotential_altitude = icao_standard_atmosphere.geopotential_altitude(geometric_altitude);
        let temperature = icao_standard_atmosphere.temperature(geopotential_altitude).expect("Layer not found.");
        let pressure = icao_standard_atmosphere.pressure(geopotential_altitude).expect("Layer not found.");
        let density = icao_standard_atmosphere.density(geopotential_altitude).expect("Layer not found.");

        file.write_fmt(format_args!("{},{},{},{}\n",geometric_altitude,temperature,pressure,density)).unwrap();
    }

}

