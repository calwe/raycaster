use crate::{HEIGHT, WIDTH};

pub struct UI;

impl UI {
    pub fn render(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let _x = i % WIDTH as usize;
            let y = i / WIDTH as usize;

            if y > HEIGHT - 150 {
                pixel.copy_from_slice(&[0x4f, 0x54, 0x6b, 0xff]);
            }
        }
    }
}
