use little_exif::exif_tag::ExifTag;
use little_exif::filetype::FileExtension;
use little_exif::ifd::ExifTagGroup;
use little_exif::metadata::Metadata;
use png::Encoder;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub fn add_metadata_to_jpeg(
    image_data: &[u8],
    metadata_input: MetadataInput,
) -> Result<Vec<u8>, JsValue> {
    let mut image_vector = image_data.to_vec();
    let mut metadata = Metadata::new_from_vec(&image_vector, FileExtension::JPEG)
        .map_err(|e| JsValue::from_str(&format!("Failed to create metadata: {:?}", e)))?;

    // longitude and latitude
    if let Some(camera_model) = metadata_input.camera_model {
        metadata.set_tag(ExifTag::Model(camera_model));
    }
    if let Some(camera_manufacturer) = metadata_input.camera_manufacturer {
        metadata.set_tag(ExifTag::Make(camera_manufacturer));
    }
    if let Some(date_created) = metadata_input.date_created {
        metadata.set_tag(ExifTag::CreateDate(date_created));
    }
    if let Some(comment) = metadata_input.comment {
        metadata.set_tag(ExifTag::UserComment(comment.into_bytes()));
    }
    if let Some(author) = metadata_input.author {
        metadata.set_tag(ExifTag::Artist(author));
    }
    if let Some(date_acquired) = metadata_input.date_acquired {
        metadata.set_tag(ExifTag::DateTimeOriginal(date_acquired));
    }
    if let Some(copyright) = metadata_input.copyright {
        metadata.set_tag(ExifTag::Copyright(copyright));
    }
    if let Some(focal_length) = metadata_input.focal_length {
        metadata.set_tag(ExifTag::FocalLength(vec![focal_length.into()]));
    }
    if let Some(saturation) = metadata_input.saturation {
        metadata.set_tag(ExifTag::Saturation(vec![(saturation as u16).into()]));
    }
    if let Some(sharpness) = metadata_input.sharpness {
        metadata.set_tag(ExifTag::Sharpness(vec![(sharpness as u16).into()]));
    }
    if let Some(white_balance) = metadata_input.white_balance {
        metadata.set_tag(ExifTag::WhiteBalance(vec![(white_balance as u16).into()]));
    }
    if let Some(f_number) = metadata_input.f_number {
        metadata.set_tag(ExifTag::FNumber(vec![f_number.into()]));
    }
    if let Some(camera_serial_number) = metadata_input.camera_serial_number {
        metadata.set_tag(ExifTag::SerialNumber(camera_serial_number));
    }
    if let Some(lens_manufacturer) = metadata_input.lens_manufacturer {
        metadata.set_tag(ExifTag::LensMake(lens_manufacturer));
    }
    if let Some(lens_model) = metadata_input.lens_model {
        metadata.set_tag(ExifTag::LensModel(lens_model));
    }
    if let Some(iso_speed) = metadata_input.iso_speed {
        metadata.set_tag(ExifTag::ISO(vec![iso_speed as u16]));
    }
    if let Some(exposure_time) = metadata_input.exposure_time {
        metadata.set_tag(ExifTag::ExposureTime(vec![exposure_time.into()]));
    }
    if let Some(exposure_bias) = metadata_input.exposure_bias {
        metadata.set_tag(ExifTag::ExposureCompensation(vec![exposure_bias.into()]));
    }
    if let Some(flash_energy) = metadata_input.flash_energy {
        metadata.set_tag(ExifTag::FlashEnergy(vec![flash_energy.into()]));
    }
    if let Some(flash) = metadata_input.flash {
        metadata.set_tag(ExifTag::Flash(vec![flash as u16]));
    }
    if let Some(focal_length_in_film) = metadata_input.focal_length_in_film {
        metadata.set_tag(ExifTag::FocalLengthIn35mmFormat(vec![
            focal_length_in_film as u16,
        ]));
    }
    if let Some(aperture) = metadata_input.aperture {
        metadata.set_tag(ExifTag::ApertureValue(vec![aperture.into()]));
    }
    if let Some(max_aperture) = metadata_input.max_aperture {
        metadata.set_tag(ExifTag::MaxApertureValue(vec![max_aperture.into()]));
    }
    if let Some(subject_distance) = metadata_input.subject_distance {
        metadata.set_tag(ExifTag::SubjectDistance(vec![subject_distance.into()]));
    }
    if let Some(metering_mode) = metadata_input.metering_mode {
        metadata.set_tag(ExifTag::MeteringMode(vec![metering_mode as u16]));
    }

    if let Some(exif_version) = metadata_input.exif_version {
        metadata.set_tag(ExifTag::ExifVersion(exif_version.into()));
    }
    if let Some(exposure_program) = metadata_input.exposure_program {
        metadata.set_tag(ExifTag::ExposureProgram(vec![exposure_program as u16]));
    }

    if let (Some(latitude), Some(longitude)) = (metadata_input.latitude, metadata_input.longitude) {
        metadata.set_tag(ExifTag::GPSLatitude(vec![latitude.into()]));
        metadata.set_tag(ExifTag::GPSLongitude(vec![longitude.into()]));

        let latitude_ref = if latitude >= 0.0 { "N" } else { "S" };
        let longitude_ref = if longitude >= 0.0 { "E" } else { "W" };
        metadata.set_tag(ExifTag::GPSLatitudeRef(latitude_ref.to_string()));
        metadata.set_tag(ExifTag::GPSLongitudeRef(longitude_ref.to_string()));
    }
    if let Some(brightness) = metadata_input.brightness {
        metadata.set_tag(ExifTag::BrightnessValue(vec![brightness.into()]));
    }
    if let Some(contrast) = metadata_input.contrast {
        metadata.set_tag(ExifTag::Contrast(vec![contrast as u16]));
    }
    if let Some(light_source) = metadata_input.light_source {
        metadata.set_tag(ExifTag::LightSource(vec![light_source as u16]));
    }
    if let Some(digital_zoom) = metadata_input.digital_zoom {
        metadata.set_tag(ExifTag::DigitalZoomRatio(vec![digital_zoom.into()]));
    }
    if let Some(shutter_speed) = metadata_input.shutter_speed {
        metadata.set_tag(ExifTag::ShutterSpeedValue(vec![shutter_speed.into()]));
    }

    if let Some(subject) = metadata_input.subject {
        let utf16_bytes: Vec<u8> = subject
            .encode_utf16()
            .flat_map(|u| u.to_le_bytes().into_iter())
            .collect();

        metadata.set_tag(ExifTag::UnknownINT8U(
            utf16_bytes,
            0x9C9F,
            ExifTagGroup::GENERIC,
        ));
    }
    if let Some(title) = metadata_input.title {
        let utf16_bytes: Vec<u8> = title
            .encode_utf16()
            .flat_map(|u| u.to_le_bytes().into_iter())
            .collect();

        metadata.set_tag(ExifTag::UnknownINT8U(
            utf16_bytes,
            0x9c9b,
            ExifTagGroup::GENERIC,
        ));
    }
    if let Some(keywords) = metadata_input.keywords {
        let utf16_bytes: Vec<u8> = keywords
            .encode_utf16()
            .flat_map(|u| u.to_le_bytes().into_iter())
            .collect();

        metadata.set_tag(ExifTag::UnknownINT8U(
            utf16_bytes,
            0x9C9E,
            ExifTagGroup::GENERIC,
        ));
    }
    if let Some(rating) = metadata_input.rating {
        metadata.set_tag(ExifTag::UnknownSTRING(
            rating,
            0x4746,
            ExifTagGroup::GENERIC,
        ));
    }

    metadata
        .write_to_vec(&mut image_vector, FileExtension::JPEG)
        .map_err(|e| JsValue::from_str(&format!("Failed to write metadata to image: {:?}", e)))?;

    Ok(image_vector)
}

pub fn add_metadata_to_png(image_data: &[u8], metadata: MetadataInput) -> Result<Vec<u8>, JsValue> {
    let img = image::load_from_memory_with_format(image_data, image::ImageFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("Failed to load PNG image: {:?}", e)))?;
    let rgba_image = img.to_rgba8();
    let (width, height) = rgba_image.dimensions();

    let mut output_buffer = Vec::new();
    {
        let mut encoder = Encoder::new(&mut output_buffer, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        // Basic metadata chunks
        if let Some(latitude) = metadata.latitude {
            encoder
                .add_text_chunk("GPS Latitude".to_string(), format!("{:.6}", latitude))
                .unwrap();
        }
        if let Some(longitude) = metadata.longitude {
            encoder
                .add_text_chunk("GPS Longitude".to_string(), format!("{:.6}", longitude))
                .unwrap();
        }
        if let Some(camera_manufacturer) = &metadata.camera_manufacturer {
            encoder
                .add_text_chunk(
                    "Camera Manufacturer".to_string(),
                    camera_manufacturer.clone(),
                )
                .unwrap();
        }
        if let Some(camera_model) = &metadata.camera_model {
            encoder
                .add_text_chunk("Camera Model".to_string(), camera_model.clone())
                .unwrap();
        }
        if let Some(date_created) = &metadata.date_created {
            encoder
                .add_text_chunk("Date Created".to_string(), date_created.clone())
                .unwrap();
        }
        if let Some(rating) = &metadata.rating {
            encoder
                .add_text_chunk("Rating".to_string(), rating.clone())
                .unwrap();
        }
        if let Some(comment) = &metadata.comment {
            encoder
                .add_text_chunk("Comment".to_string(), comment.clone())
                .unwrap();
        }
        if let Some(author) = &metadata.author {
            encoder
                .add_text_chunk("Author".to_string(), author.clone())
                .unwrap();
        }
        if let Some(date_acquired) = &metadata.date_acquired {
            encoder
                .add_text_chunk("Date Acquired".to_string(), date_acquired.clone())
                .unwrap();
        }
        if let Some(copyright) = &metadata.copyright {
            encoder
                .add_text_chunk("Copyright".to_string(), copyright.clone())
                .unwrap();
        }
        if let Some(focal_length) = metadata.focal_length {
            encoder
                .add_text_chunk(
                    "Focal Length".to_string(),
                    format!("{:.2} mm", focal_length),
                )
                .unwrap();
        }
        if let Some(saturation) = metadata.saturation {
            encoder
                .add_text_chunk("Saturation".to_string(), saturation.to_string())
                .unwrap();
        }
        if let Some(sharpness) = metadata.sharpness {
            encoder
                .add_text_chunk("Sharpness".to_string(), sharpness.to_string())
                .unwrap();
        }
        if let Some(white_balance) = metadata.white_balance {
            encoder
                .add_text_chunk("White Balance".to_string(), white_balance.to_string())
                .unwrap();
        }
        if let Some(f_number) = metadata.f_number {
            encoder
                .add_text_chunk("F Number".to_string(), format!("{:.1}", f_number))
                .unwrap();
        }
        if let Some(camera_serial_number) = &metadata.camera_serial_number {
            encoder
                .add_text_chunk(
                    "Camera Serial Number".to_string(),
                    camera_serial_number.clone(),
                )
                .unwrap();
        }
        if let Some(exposure_time) = metadata.exposure_time {
            encoder
                .add_text_chunk("Exposure Time".to_string(), format!("{:.4}", exposure_time))
                .unwrap();
        }
        if let Some(exposure_bias) = metadata.exposure_bias {
            encoder
                .add_text_chunk("Exposure Bias".to_string(), format!("{:.2}", exposure_bias))
                .unwrap();
        }
        if let Some(iso_speed) = metadata.iso_speed {
            encoder
                .add_text_chunk("ISO Speed".to_string(), iso_speed.to_string())
                .unwrap();
        }
        if let Some(flash_energy) = metadata.flash_energy {
            encoder
                .add_text_chunk("Flash Energy".to_string(), format!("{:.2}", flash_energy))
                .unwrap();
        }
        if let Some(flash) = metadata.flash {
            encoder
                .add_text_chunk("Flash".to_string(), flash.to_string())
                .unwrap();
        }
        if let Some(focal_length_in_film) = metadata.focal_length_in_film {
            encoder
                .add_text_chunk(
                    "Focal Length in 35mm Film".to_string(),
                    format!("{:.2}", focal_length_in_film),
                )
                .unwrap();
        }
        if let Some(aperture) = metadata.aperture {
            encoder
                .add_text_chunk("Aperture".to_string(), format!("{:.1}", aperture))
                .unwrap();
        }
        if let Some(max_aperture) = metadata.max_aperture {
            encoder
                .add_text_chunk("Max Aperture".to_string(), format!("{:.1}", max_aperture))
                .unwrap();
        }
        if let Some(subject_distance) = metadata.subject_distance {
            encoder
                .add_text_chunk(
                    "Subject Distance".to_string(),
                    format!("{:.2}", subject_distance),
                )
                .unwrap();
        }
        if let Some(metering_mode) = metadata.metering_mode {
            encoder
                .add_text_chunk("Metering Mode".to_string(), metering_mode.to_string())
                .unwrap();
        }
        if let Some(lens_manufacturer) = &metadata.lens_manufacturer {
            encoder
                .add_text_chunk("Lens Manufacturer".to_string(), lens_manufacturer.clone())
                .unwrap();
        }
        if let Some(lens_model) = &metadata.lens_model {
            encoder
                .add_text_chunk("Lens Model".to_string(), lens_model.clone())
                .unwrap();
        }
        if let Some(exif_version) = &metadata.exif_version {
            encoder
                .add_text_chunk("Exif Version".to_string(), exif_version.clone())
                .unwrap();
        }
        if let Some(exposure_program) = metadata.exposure_program {
            encoder
                .add_text_chunk("Exposure Program".to_string(), exposure_program.to_string())
                .unwrap();
        }
        if let Some(brightness) = metadata.brightness {
            encoder
                .add_text_chunk("Brightness".to_string(), format!("{:.2}", brightness))
                .unwrap();
        }
        if let Some(contrast) = metadata.contrast {
            encoder
                .add_text_chunk("Contrast".to_string(), contrast.to_string())
                .unwrap();
        }
        if let Some(light_source) = metadata.light_source {
            encoder
                .add_text_chunk("Light Source".to_string(), light_source.to_string())
                .unwrap();
        }
        if let Some(digital_zoom) = metadata.digital_zoom {
            encoder
                .add_text_chunk("Digital Zoom".to_string(), format!("{:.2}", digital_zoom))
                .unwrap();
        }
        if let Some(shutter_speed) = metadata.shutter_speed {
            encoder
                .add_text_chunk("Shutter Speed".to_string(), format!("{:.2}", shutter_speed))
                .unwrap();
        }
        if let Some(subject) = &metadata.subject {
            encoder
                .add_text_chunk("Subject".to_string(), subject.clone())
                .unwrap();
        }
        if let Some(keywords) = &metadata.keywords {
            encoder
                .add_text_chunk("Keywords".to_string(), keywords.clone())
                .unwrap();
        }
        if let Some(title) = &metadata.title {
            encoder
                .add_text_chunk("Title".to_string(), title.clone())
                .unwrap();
        }

        // Write header and image data
        let mut writer = encoder
            .write_header()
            .map_err(|e| JsValue::from_str(&format!("Failed to write PNG header: {:?}", e)))?;

        writer
            .write_image_data(&rgba_image)
            .map_err(|e| JsValue::from_str(&format!("Failed to write PNG data: {:?}", e)))?;
    }

    Ok(output_buffer)
}

#[wasm_bindgen]
pub fn add_metadata_to_image(image_data: &[u8], metadata_json: &str) -> Result<Vec<u8>, JsValue> {
    let metadata_input: MetadataInput = serde_json::from_str(metadata_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse metadata JSON: {:?}", e)))?;

    if image_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return add_metadata_to_jpeg(image_data, metadata_input);
    }

    return add_metadata_to_png(image_data, metadata_input);
}

#[derive(Serialize, Deserialize)]
pub struct MetadataInput {
    latitude: Option<f64>,
    longitude: Option<f64>,
    camera_manufacturer: Option<String>,
    camera_model: Option<String>,
    date_created: Option<String>,
    rating: Option<String>,
    comment: Option<String>,
    author: Option<String>,
    date_acquired: Option<String>,
    copyright: Option<String>,
    focal_length: Option<f64>,
    saturation: Option<i32>,
    sharpness: Option<i32>,
    white_balance: Option<i32>,
    f_number: Option<f64>,
    camera_serial_number: Option<String>,
    exposure_time: Option<f64>,
    exposure_bias: Option<f64>,
    iso_speed: Option<i32>,
    flash_energy: Option<f64>,
    flash: Option<i32>,
    focal_length_in_film: Option<f64>,
    aperture: Option<f64>,
    max_aperture: Option<f64>,
    subject_distance: Option<f64>,
    metering_mode: Option<i32>,
    lens_manufacturer: Option<String>,
    lens_model: Option<String>,
    exif_version: Option<String>,
    exposure_program: Option<i32>,
    brightness: Option<f64>,
    contrast: Option<i32>,
    light_source: Option<i32>,
    digital_zoom: Option<f64>,
    shutter_speed: Option<f64>,
    subject: Option<String>,
    keywords: Option<String>,
    title: Option<String>,
}
