use rusb::{Device, Result};
use std::time::Duration;

fn perform_control_transfer(device: &Device<rusb::GlobalContext>) -> Result<()> {
  let device_handle = device.open()?;
  let request_type = 0xC1;
  let request = 0x01;
  let value = 0x0100;
  let index = 0x00;
  let length = 0x14;
  let mut buf = vec![0u8; length as usize];
  let timeout = Duration::from_secs(1);

  // python: dev.ctrl_transfer(0xc1, 0x01, 0x0100, 0x00, 0x14) 
  //             ctrl_transfer(reqType, bReq, wVal, wIndex, [])
  device_handle.read_control(request_type, request, value, index, buf.as_mut_slice(), timeout)?;
  println!("Send command OK");

  Ok(())
}


fn main() -> Result<()> {
  let vendor_id = 0x045e;
  let product_id = 0x028e;
  let mut found_device = false;

  for device in rusb::devices()?.iter() {
    let device_desc = device.device_descriptor()?;

    println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
      device.bus_number(),
      device.address(),
      device_desc.vendor_id(),
      device_desc.product_id());

    if device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id {
      found_device = true;
      println!("Xbox Controller found");
      perform_control_transfer(&device)?;
      break;
    }
  }

  if !found_device {
    print!("Device not found");
  }

  Ok(())
}
