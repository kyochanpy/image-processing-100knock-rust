use image::{Rgb, RgbImage, GenericImageView};

pub fn run(base_image_path: &str, result_image_path: &str) {
    // 画像読み込み
    let img = image::open(base_image_path).unwrap();

    // 新しい画像を作成
    let mut result_img = RgbImage::new(img.width(), img.height());

    // ピクセル単位で処理
    for (x, y, pixel) in result_img.enumerate_pixels_mut() {
        let rgb = img.get_pixel(x, y);
        *pixel = Rgb([rgb[2], rgb[1], rgb[0]]);
    }

    // 保存
    result_img.save(result_image_path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let base_image_path = "src/images/imori.jpg";
        let result_image_path = "src/images/result001.jpg";
        run(base_image_path, result_image_path);
    }
}
