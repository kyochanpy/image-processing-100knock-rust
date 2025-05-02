use crate::questions::q002::rgb_to_gray;
use image::{DynamicImage, Luma};

pub fn binarization(base_image_path: &str) -> DynamicImage {
    let mut result = rgb_to_gray(base_image_path).to_luma8();

    result.pixels_mut().for_each(|pixel| {
        let gray = pixel[0];
        *pixel = Luma([if gray >= 128 { 255 } else { 0 }]);
    });
    DynamicImage::ImageLuma8(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::questions::test_image_processing;

    #[test]
    fn test_run() {
        test_image_processing(
            binarization,
            "src/images/imori.png",
            "src/images/answers/answer_003.png",
        );
    }
}
