use derive_new::new;

#[repr(C)]
#[derive(new, Default)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}
