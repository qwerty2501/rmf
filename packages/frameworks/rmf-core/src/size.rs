use derive_new::new;

#[repr(C)]
#[derive(new)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}
