use png::{
    BitDepth,
    ColorType,
    Encoder,
    EncodingError,
};
use std::io::Cursor;
use swash::scale::image::Image;

/// Encodes a raster image into 8-bit PNG format.
///
/// # Arguments
/// - `image`: The [`Image`] instance.
///
/// # Returns
/// - [`Vec<u8>`]: PNG data on success.
/// - [`EncodingError`] on failure.
///
/// [`Image`]: swash::scale::image::Image
pub(crate) fn encode_image(image: &Image) -> Result<Vec<u8>, EncodingError> {
    let mut out = Vec::new();
    let cursor = Cursor::new(&mut out);

    let mut encoder = Encoder::new(cursor, image.placement.width, image.placement.height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);

    {
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&image.data)?;
    }

    Ok(out)
}
