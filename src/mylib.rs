use image::DynamicImage;
use image::imageops;

pub struct image_processor {
    image_resolution: u32,
}

impl image_processor {

    pub fn new(val: u32) -> Self {
        return image_processor {
            image_resolution: val,
        };
    }

    fn preprocess(&self, img: DynamicImage) -> image::RgbaImage {
        let (width, height) = (img.width(), img.height());
        let size = width.min(height);
        let x = (width - size) / 2;
        let y = (height - size) / 2;
        let cropped_img = imageops::crop_imm(&img, x, y, size, size).to_image();
        imageops::resize(
            &cropped_img,
            self.image_resolution,
            self.image_resolution,
            imageops::FilterType::CatmullRom,
        )
    }

    pub fn decode_and_preprocess(&self, data: Vec<u8>) -> Result<image::RgbaImage, String> {
        match image::load_from_memory(&data) {
            Ok(img) => {
                return Ok(self.preprocess(img));
            }
            Err(e) => {
                return Err("decode error".to_string());
            }
        };
    }
}
