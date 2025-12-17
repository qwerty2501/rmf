use derive_new::new;

#[repr(C)]
#[derive(new, Default, Debug)]
pub struct Size {
    pub height: u32,
    pub width: u32,
}
