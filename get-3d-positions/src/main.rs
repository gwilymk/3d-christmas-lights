use image::{save_buffer, ColorType, ImageBuffer, Rgb};
use nokhwa::{Camera, FrameFormat};

use imageproc::filter::gaussian_blur_f32;
use serialport::ClearBuffer;

use std::error::Error;
use std::fs::File;
use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut camera = Camera::new(0, None)?;
    let all_formats = camera.compatible_list_by_resolution(FrameFormat::MJPEG)?;

    let best_format = all_formats
        .iter()
        .max_by(|x, y| x.0.x().cmp(&y.0.height()))
        .unwrap();

    camera.set_resolution(*best_format.0)?;

    let format = camera.camera_format();

    let mut serial = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(20))
        .open()?;

    println!(
        "{}x{}, {}",
        format.width(),
        format.height(),
        format.format(),
    );

    camera.open_stream()?;

    let mut output = File::create("target/output.dat")?;

    for i in 0..100 {
        print!("Next?");
        stdout().flush()?;

        print!("Taking for {}... ", i);
        stdout().flush()?;
        thread::sleep(Duration::from_millis(100));
        let pos = take_image(&mut camera, i)?;
        println!("Done");

        println!("Clearing buffers");
        serial.clear(ClearBuffer::All)?;
        serial.clear(ClearBuffer::All)?;
        serial.clear(ClearBuffer::All)?;
        serial.clear(ClearBuffer::All)?;
        serial.clear(ClearBuffer::All)?;
        serial.clear(ClearBuffer::All)?;
        serial.clear(ClearBuffer::All)?;
        println!("Starting next image");
        serial.write(&[i as u8])?;
        serial.flush()?;

        writeln!(&mut output, "{} = {},{}", i, pos.0, pos.1)?;
    }

    Ok(())
}

fn take_image(camera: &mut Camera, index: i32) -> Result<(usize, usize), Box<dyn Error>> {
    let frame = camera.frame()?;

    let mut frame: ImageBuffer<Rgb<u8>, Vec<_>> = gaussian_blur_f32(&frame, 3.0);

    // the light is white, so expecting a low standard deviation too
    let mut maximum_brightness = 0;
    let mut best_pos = None;
    let mut colour_of_best = None;

    for (i, row) in frame.rows().enumerate() {
        for (j, pixel) in row.enumerate() {
            let brightness = pixel.0.iter().fold(0, |prev, &next| prev + next as i32);

            if brightness > maximum_brightness {
                maximum_brightness = brightness;
                best_pos = Some((j, i));
                colour_of_best = Some(*pixel);
            }
        }
    }

    println!("colour of best = {:?}", colour_of_best);

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
        format!("target/result-{:03}.png", index),
        &frame,
        frame.width(),
        frame.height(),
        ColorType::Rgb8,
    )?;

    Ok(best_pos)
}
