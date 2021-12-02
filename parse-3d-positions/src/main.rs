use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn get_coords(filename: &str) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut result: Vec<_> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let coord = line
                .split(' ')
                .last()
                .unwrap()
                .split(',')
                .collect::<Vec<_>>();

            let x: i32 = coord[0].parse().unwrap();
            let y: i32 = coord[1].parse().unwrap();

            (x, y)
        })
        .collect();

    // I screwed up in the generation code so first = last. So make the first equal to the second
    result[0] = result[1];

    Ok(result)
}

fn fudge_coords(first_coords: Vec<(i32, i32)>, third_coords: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // these should ideally be a mirror image of each other.
    // find the best match in the y axis, and use that to find the mirror point in the x
    let mut best_y = 0;
    let mut best_y_value = 100;
    for (i, (first, third)) in first_coords.iter().zip(third_coords.iter()).enumerate() {
        let y_diff = (first.1 - third.1).abs();

        if y_diff < best_y_value {
            best_y_value = y_diff;
            best_y = i;
        }

        if y_diff > 60 {
            println!("{} seems dodgy", i);
        }
    }

    println!("Using {} as the example for best combination", best_y);

    let x_offset_first = first_coords[best_y].0;
    let x_offset_third = third_coords[best_y].1;

    let fudged_first = first_coords.iter().map(|(x, y)| (x - x_offset_first, *y));
    let fudged_third = third_coords.iter().map(|(x, y)| (x_offset_third - x, *y));

    fudged_first
        .zip(fudged_third)
        .map(|((x1, y1), (x2, y2))| ((x1 + x2) / 2, (y1 + y2) / 2))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let first_coords = get_coords("../get-3d-positions/first/output.dat")?;
    let third_coords = get_coords("../get-3d-positions/third/output.dat")?;

    let second_coords = get_coords("../get-3d-positions/second/output.dat")?;
    let fourth_coords = get_coords("../get-3d-positions/fourth/output.dat")?;

    let x_facing_coords = fudge_coords(first_coords, third_coords);
    let z_facing_coords = fudge_coords(second_coords, fourth_coords);

    let xyzs: Vec<_> = x_facing_coords
        .iter()
        .zip(z_facing_coords.iter())
        .enumerate()
        .map(|(i, ((x, y1), (z, y2)))| {
            if (y1 - y2).abs() > 60 {
                println!("{} looks very dodgy", i);
            }

            (*x, (y1 + y2) / 2, *z)
        })
        .collect();

    let mut output = File::create("output.csv")?;
    let mut output_c = File::create("../lights.h")?;

    writeln!(&mut output_c, "static int light_positions[] = {{")?;

    for xyz in xyzs {
        writeln!(&mut output, "{},{},{}", xyz.0, xyz.1, xyz.2)?;
        writeln!(&mut output_c, "    {}, {}, {},", xyz.0, xyz.1, xyz.2)?;
    }

    writeln!(&mut output_c, "}};")?;

    Ok(())
}
