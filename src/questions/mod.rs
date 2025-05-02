pub mod q001;
pub mod q002;
pub mod q003;

use image::DynamicImage;

pub fn test_image_processing<F>(processing_fn: F, base_image_path: &str, answer_image_path: &str)
where
    F: FnOnce(&str) -> DynamicImage,
{
    let result = processing_fn(base_image_path);
    let answer = image::open(answer_image_path).unwrap();
    assert_eq!(result, answer);
}
