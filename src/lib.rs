mod model;

use model::{PositionData, RotationData};

#[link(name = "XRSDK", kind = "dylib")]
extern "C" {
  fn XRSDK_Init();
  fn XRSDK_Shutdown();
  fn GetArSensor() -> *mut u8;
}

static mut POS_DATA: PositionData = PositionData {
  enabled: false,
  x: 0.0,
  y: 0.0,
  z: 0.0,
};

static mut ROT_DATA: RotationData = RotationData {
  enabled: false,
  x: 0.0,
  y: 0.0,
  z: 0.0,
  w: 0.0,
};

pub fn init() {
  unsafe { XRSDK_Init() }
}

pub fn shutdown() {
  unsafe { XRSDK_Shutdown() }
}

pub unsafe fn get_position() -> *mut PositionData {
  POS_DATA.enabled = false;
  &mut POS_DATA
}

pub unsafe fn get_rotation() -> *mut RotationData {
  ROT_DATA.enabled = false;
  unsafe {
    let p = GetArSensor();
    // read float from p+44
    ROT_DATA.x = *(p.add(44)) as f32;
    ROT_DATA.y = *(p.add(48)) as f32;
    ROT_DATA.z = *(p.add(52)) as f32;
    ROT_DATA.w = *(p.add(56)) as f32;
  }
  &mut ROT_DATA
}
