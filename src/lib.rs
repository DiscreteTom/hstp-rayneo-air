#[link(name = "XRSDK", kind = "dylib")]
extern "C" {
  fn XRSDK_Init();
  fn XRSDK_Shutdown();
  fn GetArSensor() -> *mut u8;
}

/// enabled: bool, x: f32, y: f32, z: f32
static mut POS_DATA: [u8; 13] = [0; 13];
/// enabled: bool, x: f32, y: f32, z: f32, w: f32
static mut ROT_DATA: [u8; 17] = [0; 17];

pub fn init() {
  unsafe {
    ROT_DATA[0] = 1; // enable rotation
    XRSDK_Init()
  }
}

pub fn shutdown() {
  unsafe {
    ROT_DATA[0] = 0; // disable rotation
    XRSDK_Shutdown()
  }
}

pub unsafe fn get_position() -> *mut [u8; 13] {
  &mut POS_DATA // position is disabled
}

pub unsafe fn get_rotation() -> *mut [u8; 17] {
  unsafe {
    let p = GetArSensor();
    // read float from p+44
    p.add(44).copy_to(ROT_DATA.as_mut_ptr().add(1), 4);
    p.add(48).copy_to(ROT_DATA.as_mut_ptr().add(5), 4);
    p.add(52).copy_to(ROT_DATA.as_mut_ptr().add(9), 4);
    p.add(56).copy_to(ROT_DATA.as_mut_ptr().add(13), 4);
  }
  &mut ROT_DATA
}
