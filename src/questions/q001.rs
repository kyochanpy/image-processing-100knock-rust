use image::{DynamicImage, GenericImageView, Rgb, RgbImage};

pub fn rgb_to_bgr(base_image_path: &str) -> DynamicImage {
    let img = image::open(base_image_path).unwrap();
    let mut result_img = RgbImage::new(img.width(), img.height());

    for (x, y, pixel) in result_img.enumerate_pixels_mut() {
        let rgb = img.get_pixel(x, y);
        *pixel = Rgb([rgb[2], rgb[1], rgb[0]]);
    }
    DynamicImage::ImageRgb8(result_img)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::questions::test_image_processing;

    #[test]
    fn test_run() {
        test_image_processing(
            rgb_to_bgr,
            "src/images/imori.png",
            "src/images/answers/answer_001.png",
        );
    }
}
