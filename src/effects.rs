use image::{DynamicImage, GenericImage, Rgba, GenericImageView};
use queues::{Queue, queue, IsQueue};

use crate::utils;

pub fn is_supported_effect(effect: &str) -> bool {
   if effect.to_lowercase() == "grayscale"   ||
      effect.to_lowercase() == "edgedetect"  ||
      effect.to_lowercase() == "invertcolor" ||
      effect.to_lowercase() == "floodfill" {
      return true;
   }

   false
}

pub fn grayscale(image: &mut DynamicImage) {
   let pixels = utils::get_pixel_vec(image);

   for pixel in pixels {
      let rgb = pixel.data.0;

      // u16 here so we don't overflow before the division by 3.
      let mut gray: u16 = rgb[0] as u16;
      gray += rgb[1] as u16;
      gray += rgb[2] as u16;
      gray /= 3;

      let mut updated_rgba = pixel.data.clone();
      updated_rgba.0[0] = gray as u8;
      updated_rgba.0[1] = gray as u8;
      updated_rgba.0[2] = gray as u8;

      image.put_pixel(pixel.row_index, pixel.col_index, updated_rgba);
   }
}

// Get brightness of a color
fn luminance(color: Rgba<u8>) -> f64 {
   let red = color.0[0];
   let green = color.0[1];
   let blue = color.0[2];

   return 0.299 * red as f64 + 0.587 * green as f64 + 0.114 * blue as f64;
}

// Finds edges of a picture
pub fn edge_detect(image: &mut DynamicImage) {
	grayscale(image);

	// Sobel operators\
   let horizontal:[[i32;3];3] = [
      [-1, 0, 1],
      [-2, 0, 2],
      [-1, 0, 1]
   ];

   let vertical:[[i32;3];3] = [
      [1, 2, 1],
      [0, 0, 0],
      [-1, -2, -1]
   ];

   let pixels = utils::get_pixel_vec(image);

   for pixel in pixels.as_slice() {
      if pixel.col_index == 0 || pixel.row_index == 0  ||
         pixel.col_index == image.height() - 1 || pixel.row_index == image.width() - 1{
         continue;
      }

      let mut gradient: [[i32; 3]; 3] = [[0; 3]; 3];

      let x = pixel.row_index;
      let y = pixel.col_index;

      for i in 0..3 {
         for j in 0..3 {
            let index: usize = (x - 1 + i) as usize * image.height() as usize + (y - 1 + j) as usize;
            let pixel_at_index = pixels.get(index);
            if let Some(value) = pixel_at_index {
               gradient[i as usize][j as usize] = luminance(value.data) as i32;
            }
         }
      }

      let mut gradient_x = 0;
      let mut gradient_y = 0;

      for i in 0..3 {
         for j in 0..3 {
            gradient_y += gradient[i][j] * horizontal[i][j];
            gradient_x += gradient[i][j] * vertical[i][j];
         }
      }

      let tmp = ((gradient_x * gradient_x) + (gradient_y * gradient_y)) as f64;
      let color = tmp.sqrt();

      let mut updated_rgba = pixel.data.clone();
      updated_rgba.0[0] = color as u8;
      updated_rgba.0[1] = color as u8;
      updated_rgba.0[2] = color as u8;

      image.put_pixel(x, y, updated_rgba);
   }
}

pub fn flood_fill(image: &mut DynamicImage, row: i32, col: i32, color: Rgba<u8>) {
   if row as u32 >= image.height() || col as u32 >= image.width() {
      panic!("Out of bounds pixel index passed (X: {}, Y: {}) with bounds (X: {}, Y: {})",
             row, col, image.height(), image.width());
   }

   // init the state
   let xd: [i32; 8] = [1, -1, 0, 0, 1, 1, -1, -1];
   let yd: [i32; 8] = [0, 0, 1, -1, 1, -1, 1, -1];

   let mut visited: Vec<Vec<bool>> = Vec::new();

   for _ in 0..image.width() {
      let mut vec: Vec<bool> = Vec::new();
      for _ in 0..image.height() {
         vec.push(false);
      }

      visited.push(vec);
   }

   let mut coords: Queue<(i32, i32)> = queue![];

	// For all neighbours of row, col
	for i in 0..8 {
		// Check if they are inside the boundaries
		if row + xd[i] < 0 ||
         row + xd[i] > image.width() as i32 ||
         col + yd[i] < 0 ||
         col + yd[i] > image.height() as i32 {
			continue
		} else {
			// Check if neighbours color is the same
         if image.get_pixel(row as u32, col as u32) == image.get_pixel((row + xd[i]) as u32, (col + xd[i]) as u32) {
            coords.add((row + xd[i], col + yd[i])).unwrap();
         }
		}
	}

	// Set color of row,col and mark as visited
   image.put_pixel(row as u32, col as u32, color);
	visited[row as usize][col as usize] = true;

   while coords.size() != 0 {
      let res = coords.peek();

      match res {
         Ok(current_pixel) => {
            for i in 0..8 {
               if current_pixel.0 + xd[i] < 0 ||
                  current_pixel.0 + xd[i] >= image.width() as i32 ||
                  current_pixel.1 + yd[i] < 0 ||
                  current_pixel.1 + yd[i] >= image.height() as i32 {
                  continue;
               } else {
                  if (image.get_pixel(current_pixel.0 as u32, current_pixel.1 as u32) ==
                      image.get_pixel((current_pixel.0 + xd[i]) as u32, (current_pixel.1 + yd[i]) as u32)) &&
                     !visited[(current_pixel.0 + xd[i]) as usize][(current_pixel.1 + yd[i]) as usize] {
                        coords.add((current_pixel.0 + xd[i], current_pixel.1 + yd[i])).unwrap();
                        visited[(current_pixel.0 + xd[i]) as usize][(current_pixel.1 + yd[i]) as usize] = true;
                  }
               }
            }

            image.put_pixel(current_pixel.0 as u32, current_pixel.1 as u32, color);
            let _ = coords.remove();
         },
         Err(_) => { break; /* Queue is empty. */ }
      }
   }
}

pub fn invert_color(image: &mut DynamicImage) {
   let pixels = utils::get_pixel_vec(image);

   for pixel in pixels {
      let mut new_rgba = pixel.data.clone();
      new_rgba.0[0] = 255 - new_rgba.0[0];
      new_rgba.0[1] = 255 - new_rgba.0[1];
      new_rgba.0[2] = 255 - new_rgba.0[2];

      image.put_pixel(pixel.row_index, pixel.col_index, new_rgba);
   }
}