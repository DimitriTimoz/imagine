use druid::{widget::Image, ImageBuf};
use crate::prelude::*;

pub fn build_image_obj() -> Image {
    let image_buf = ImageBuf::from_file("/Users/dimitri/Documents/image.png").unwrap();
    Image::new(image_buf)
}
