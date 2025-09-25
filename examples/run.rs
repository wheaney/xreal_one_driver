use xreal_one_driver::XrealOne;

fn main() {
    let mut xreal_one = match XrealOne::new() {
        Ok(xreal_one) => xreal_one,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    loop {
        let imu = match xreal_one.next() {
            Ok(imu) => imu,
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            }
        };
        println!("IMU: {:?}", imu);
    }
}
