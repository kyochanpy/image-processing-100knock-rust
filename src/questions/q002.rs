use image::{DynamicImage, GenericImageView, GrayImage, Luma};

pub fn rgb_to_gray(base_image_path: &str) -> DynamicImage {
    let img = image::open(base_image_path).unwrap();
    let mut result_img = GrayImage::new(img.width(), img.height());

    for (x, y, pixel) in result_img.enumerate_pixels_mut() {
        let pix = img.get_pixel(x, y);
        let gray = Luma([
            (pix[0] as f32 * 0.2126 + pix[1] as f32 * 0.7152 + pix[2] as f32 * 0.0722) as u8,
        ]);
        *pixel = gray;
    }
    DynamicImage::ImageLuma8(result_img)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::questions::test_image_processing;

    #[test]
    fn test_run() {
        test_image_processing(
            rgb_to_gray,
            "src/images/imori.png",
            "src/images/answers/answer_002.png",
        );
    }
}
