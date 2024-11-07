# turbo_exif.js
A library for writing metadata to jpg / webp / tiff / png. Unlike the javascript libraries out there,
that convert to base64 and still aren't able to add metadata to the image, or are limited to formats to which they can add
metadata to, the turbo_exif.js is much faster compared to libraries like piexifjs by 2-3x and can add metadata to jpeg, tiff, png or webp formats.

## Advantages of turbo_exif.js:
1. Fastest Exif / Metadata writer for images in JS.
2. Supports multiple formats (PNG / JPG / TIFF / Webp).
3. Non-blocking (Runs in WebAssembly).

## Sample JS Code Snippet
Refer to the code below to add metadata to the images. For a working demo, refer to the steps below

```js
const metadata = {
    lens_manufacturer: "Canon",
    lens_model: "EF 50mm f/1.4 USM",
    exif_version: "0230",
    exposure_program: 2,
    brightness: 0.5,
    subject: "Nature",
    keywords: "photography, nature, outdoor",
    title: "Beautiful Landscape",
    latitude: 23.45,
    longitude: -75.12,
};

const outputUint8Array = add_metadata_to_image(
    imageBytes,
    JSON.stringify(metadata),
);
```

## Running the Demo

Follow these steps:

1. Copy the pkg folder in html-sample.
2. cd html-sample
3. Run the command python3 -m http.server 8000
4. Open localhost:8000/exif-writer.html
5. Upload image in the html page and download image and verify metadata using any tool.

## Metadata Supported

The following metadata fields are supported.

| Field                  | Type           | Description                               |
|------------------------|----------------|-------------------------------------------|
| `latitude`             | `Option<f64>`  | Latitude coordinate of the image location. |
| `longitude`            | `Option<f64>`  | Longitude coordinate of the image location. |
| `camera_manufacturer`  | `Option<String>` | Camera manufacturer name.                 |
| `camera_model`         | `Option<String>` | Camera model name.                        |
| `date_created`         | `Option<String>` | Date the image was created.               |
| `rating`               | `Option<String>` | User-defined rating for the image.        |
| `comment`              | `Option<String>` | Comment or note about the image.          |
| `author`               | `Option<String>` | Author or photographer's name.            |
| `date_acquired`        | `Option<String>` | Date the image was acquired.              |
| `copyright`            | `Option<String>` | Copyright information.                    |
| `focal_length`         | `Option<f64>`  | Focal length used in the image.           |
| `saturation`           | `Option<i32>`  | Saturation level setting.                 |
| `sharpness`            | `Option<i32>`  | Sharpness level setting.                  |
| `white_balance`        | `Option<i32>`  | White balance setting.                    |
| `f_number`             | `Option<f64>`  | Aperture size (f-number) of the lens.     |
| `camera_serial_number` | `Option<String>` | Serial number of the camera.              |
| `exposure_time`        | `Option<f64>`  | Exposure time in seconds.                 |
| `exposure_bias`        | `Option<f64>`  | Exposure bias value.                      |
| `iso_speed`            | `Option<i32>`  | ISO speed setting.                        |
| `flash_energy`         | `Option<f64>`  | Flash energy setting.                     |
| `flash`                | `Option<i32>`  | Flash status (on/off).                    |
| `focal_length_in_film` | `Option<f64>`  | Focal length in 35mm film equivalent.     |
| `aperture`             | `Option<f64>`  | Aperture value used.                      |
| `max_aperture`         | `Option<f64>`  | Maximum aperture of the lens.             |
| `subject_distance`     | `Option<f64>`  | Distance to the subject.                  |
| `metering_mode`        | `Option<i32>`  | Metering mode setting.                    |
| `lens_manufacturer`    | `Option<String>` | Lens manufacturer name.                   |
| `lens_model`           | `Option<String>` | Lens model name.                          |
| `exif_version`         | `Option<String>` | EXIF version used.                        |
| `exposure_program`     | `Option<i32>`  | Exposure program mode.                    |
| `brightness`           | `Option<f64>`  | Brightness value of the image.            |
| `contrast`             | `Option<i32>`  | Contrast level setting.                   |
| `light_source`         | `Option<i32>`  | Light source used.                        |
| `digital_zoom`         | `Option<f64>`  | Digital zoom ratio.                       |
| `shutter_speed`        | `Option<f64>`  | Shutter speed value.                      |
| `subject`              | `Option<String>` | Subject of the image.                     |
| `keywords`             | `Option<String>` | Keywords associated with the image.       |
| `title`                | `Option<String>` | Title of the image.                       |

## Building the source code

```
# Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
cargo install wasm-pack

# Compile package
wasm-pack build --target web

# Follow steps above to run sample.

# Testing rust code directly
cargo run
```
