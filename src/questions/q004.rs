use crate::questions::q002::rgb_to_gray;
use image::{DynamicImage, Luma};

pub fn otsu_binarization(base_image_path: &str) -> DynamicImage {
    let mut result = rgb_to_gray(base_image_path).to_luma8();

    let mut best_threshold = 0;
    let mut max_variance = 0.0;

    for threshold in 0..255 {
        let mut sum_below: u32 = 0;
        let mut sum_above: u32 = 0;
        let mut count_below: u32 = 0;
        let mut count_above: u32 = 0;

        for pixel in result.pixels() {
            if pixel[0] < threshold {
                sum_below += pixel[0] as u32;
                count_below += 1;
            } else {
                sum_above += pixel[0] as u32;
                count_above += 1;
            }
        }

        let mean_below = sum_below as f32 / count_below as f32;
        let mean_above = sum_above as f32 / count_above as f32;

        let variance = count_below as f32 * count_above as f32 * (mean_below - mean_above).powi(2)
            / (count_below + count_above) as f32;

        if variance > max_variance {
            max_variance = variance;
            best_threshold = threshold;
        }
    }

    result.pixels_mut().for_each(|pixel| {
        let gray = pixel[0];
        *pixel = Luma([if gray >= best_threshold { 255 } else { 0 }]);
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
            otsu_binarization,
            "src/images/imori.png",
            "src/images/answers/answer_004.png",
        );
    }
}
