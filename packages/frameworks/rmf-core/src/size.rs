use derive_new::new;

#[repr(C)]
#[derive(new, Default, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
