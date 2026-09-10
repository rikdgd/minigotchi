use macroquad::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;


/// This example can be used to generate the byte data used by the [miniquad Icon](macroquad::miniquad::conf::Icon).
#[macroquad::main("gen icon bytes")]
async fn main() {
    let image_pixels = load_image_pixel_data().await;
    let image_pixel_bytes = pixels_to_small_icon_data(image_pixels).unwrap();
    let img_bytes_text = format_image_bytes(image_pixel_bytes);
    
    // Write the result to a file for easier reusage.
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("image_byte_data.txt")
        .unwrap();
    
    file.write_all(img_bytes_text.as_bytes()).unwrap();
    file.flush().unwrap();
}


async fn load_image_pixel_data() -> Vec<[u8; 4]> {
    let image = load_image("resources/icon/icon_16x16.png").await.unwrap();
    let image_pixels = image.get_image_data();
    image_pixels.to_vec()
}

/// This function is used to convert an image's pixel data to a flattened array with the exact length required
/// by the [miniquad icon](macroquad::miniquad::conf::Icon) for a small sized icon.
fn pixels_to_small_icon_data(pixel_data: Vec<[u8; 4]>) -> Result<[u8; 1024], String> {
    if pixel_data.len() != 16 * 16 {
        return Err("Image doesn't contain exactly 16x16 pixels".to_string());
    }
    
    let mut buffer = [0u8; 1024];
    let mut index = 0;
    
    for pixel in pixel_data {
        for byte in pixel {
            buffer[index] = byte;
            index += 1;
        }
    }
    
    Ok(buffer)
}

/// Formats an array of byte data from a 16x16 (r,g,b,a) image into a string to be written to a file.
fn format_image_bytes(image_bytes: [u8; 1024]) -> String {
    let bytes_text = format!("{:?}", image_bytes);
    let mut space_counter = 0;
    let mut formatted_byte_text = String::new();
    
    for c in bytes_text.chars() {
        if c == ' ' {
            space_counter += 1;
        }
        
        if space_counter == 16 * 4 {
            space_counter = 0;
            formatted_byte_text.push('\n');
        } else {
            formatted_byte_text.push(c);
        }
    }
    
    formatted_byte_text
}