// src/main.rs
use serde_json::json;
use std::fs;
use std::io::Read;
use turbo_exif::add_metadata_to_image; // Replace `your_crate_name` with your actual crate name

fn main() {
    let image_path = "/Users/kartikdutt/Downloads/input.jpeg";
    let output_path = "/Users/kartikdutt/Downloads/output.jpeg";

    let mut file = fs::File::open(image_path).expect("Failed to open image file");
    let mut image_data = Vec::new();
    file.read_to_end(&mut image_data)
        .expect("Failed to read image data");

    let metadata_json = json!({
        "latitude": 40.7128,
        "longitude": -74.0060,
        "camera_manufacturer": "Canon",
        "camera_model": "EOS 5D Mark IV",
        "date_created": "2023-11-06",
        "rating": "5",
        "comment": "Sample photo for testing.",
        "author": "John Doe",
        "date_acquired": "2023-11-07",
        "copyright": "John Doe 2023",
        "focal_length": 50.0,
        "saturation": 2,
        "sharpness": 3,
        "white_balance": 1,
        "f_number": 1.8,
        "camera_serial_number": "123456789",
        "exposure_time": 0.008,
        "exposure_bias": 0.0,
        "iso_speed": 800,
        "flash_energy": 1.0,
        "flash": 0,
        "focal_length_in_film": 50.0,
        "aperture": 2.8,
        "max_aperture": 1.4,
        "subject_distance": 2.0,
        "metering_mode": 1,
        "lens_manufacturer": "Canon",
        "lens_model": "EF 50mm f/1.4 USM",
        "exif_version": "0230",
        "exposure_program": 2,
        "brightness": 0.5,
        "contrast": 1,
        "light_source": 1,
        "digital_zoom": 1.0,
        "shutter_speed": 1.0 / 2000.0,
        "subject": "Nature",
        "keywords": "photography, nature, outdoor",
        "title": "Beautiful Landscape"
    })
    .to_string();

    match add_metadata_to_image(&image_data, &metadata_json) {
        Ok(output_data) => {
            fs::write(output_path, output_data).expect("Failed to write output image");
            println!("Metadata added successfully!");
        }
        Err(e) => {
            eprintln!("Error processing image: {:?}", e);
        }
    }
}
