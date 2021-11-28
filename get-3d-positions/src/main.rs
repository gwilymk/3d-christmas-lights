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
    let frame = camera.frame()?;

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

    println!("Best pos: {}, {}", best_pos.unwrap().0, best_pos.unwrap().1);

    Ok(())
}
