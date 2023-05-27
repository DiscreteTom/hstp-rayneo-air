mod model;

use model::{PositionData, RotationData};

#[link(name = "XRSDK", kind = "dylib")]
extern "C" {
  fn XRSDK_Init();
  fn XRSDK_Shutdown();
  fn GetArSensor() -> *mut u8;
}

pub fn init() {
  unsafe { XRSDK_Init() }
}

pub fn shutdown() {
  unsafe { XRSDK_Shutdown() }
}

pub fn get_position(data: &mut PositionData) {
  data.enabled = false;
}

pub fn get_rotation(data: &mut RotationData) {
  data.enabled = false;
  unsafe {
    let p = GetArSensor();
    // read float from p+44
    data.x = *(p.add(44)) as f32;
    data.y = *(p.add(48)) as f32;
    data.z = *(p.add(52)) as f32;
    data.w = *(p.add(56)) as f32;
  }
}
