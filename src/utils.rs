use image::{Rgba, DynamicImage, GenericImageView};

pub struct RgbaPixel {
   pub row_index: u32,
   pub col_index: u32,
   pub data: Rgba<u8>
}

impl RgbaPixel {
   pub fn new(row_index: u32, col_index: u32, data: Rgba<u8>) -> Self {
      RgbaPixel {
         row_index,
         col_index,
         data
      }
   }
}

pub fn get_pixel_vec(image: &DynamicImage) -> Vec<RgbaPixel> {
   let mut res: Vec<RgbaPixel> = Vec::new();

   for row in 0..image.width() {
      for col in 0..image.height() {
          let pixel = image.get_pixel(row, col);

          res.push(RgbaPixel::new(row, col, pixel));
      }
   }

  res
}