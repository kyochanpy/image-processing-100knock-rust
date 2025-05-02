use image::DynamicImage;

pub fn rgb_to_bgr(base_image_path: &str) -> DynamicImage {
    let img = image::open(base_image_path).unwrap();
    let mut rgb = img.to_rgb8();

    rgb.pixels_mut().for_each(|pixel| {
        let r = pixel[0];
        let b = pixel[2];
        pixel[0] = b;
        pixel[2] = r;
    });
    DynamicImage::ImageRgb8(rgb)
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
