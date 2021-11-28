use image::{save_buffer, ColorType, Rgb};
use nokhwa::{Camera, FrameFormat};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut camera = Camera::new(0, None)?;
    let all_formats = camera.compatible_list_by_resolution(FrameFormat::MJPEG)?;

    let best_format = all_formats
        .iter()
        .max_by(|x, y| x.0.x().cmp(&y.0.height()))
        .unwrap();

    camera.set_resolution(*best_format.0)?;

    let format = camera.camera_format();

    println!(
        "{}x{}, {}",
        format.width(),
        format.height(),
        format.format()
    );

    camera.open_stream()?;
    let mut frame = camera.frame()?;

    let mut maximum_brightness = 0;
    let mut best_pos = None;

    for (i, row) in frame.rows().enumerate() {
        for (j, pixel) in row.enumerate() {
            let brightness = pixel.0.iter().fold(0, |prev, &next| prev + next as i32);

            if brightness > maximum_brightness {
                maximum_brightness = brightness;
                best_pos = Some((j, i));
            }
        }
    }

    let best_pos = best_pos.unwrap();
    let best_x = best_pos.0;
    let best_y = best_pos.1;

    let red = Rgb([255, 0, 0]);

    for y in best_y.saturating_sub(10)..best_y.saturating_add(10).min(frame.height() as usize) {
        frame.put_pixel(best_x as u32, y as u32, red);
    }

    for x in best_x.saturating_sub(10)..best_x.saturating_add(10).min(frame.width() as usize) {
        frame.put_pixel(x as u32, best_y as u32, red);
    }

    save_buffer(
        "result.png",
        &frame,
        frame.width(),
        frame.height(),
        ColorType::Rgb8,
    )?;

    Ok(())
}
