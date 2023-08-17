use std::error::Error;

use libnspire::{PID_CX2, Handle};

fn main() -> Result<(), Box<dyn Error>> {
    let devices = rusb::devices()?;
    for dev in devices.iter() {
        let desc = dev.device_descriptor()?;
        if desc.product_id() == PID_CX2 {
            println!("Found a CX II!");
            let handle = Handle::new(dev.open()?)?;
            handle.write_file("/Press-to-Test/Exit Test Mode.tns", include_bytes!("Exit Test Mode.tns"), &mut |_| {})?;
        }
    }
    Ok(())
}
