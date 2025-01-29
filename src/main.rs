use clap::{error::Result, Parser};
use image::{Rgb, RgbImage};

/// Simple CLI tool to create a color palette texture from a list of hex colors
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hex Colors (comma separated)
    #[arg(required = true, value_parser = parse_colors, num_args = 1.., value_delimiter = ' ')]
    colors: Box<[Rgb<u8>]>,

    /// Size of palette in pixels
    #[arg(short, long, default_value_t = 5)]
    size: u32,

    /// Output path
    #[arg(short, long, default_value = "palette-texture.png")]
    out: String,
}

fn parse_colors(s: &str) -> Result<Box<[Rgb<u8>]>, String> {
    let mut colors = vec![];
    for c in s.split(',') {
        let c = c.trim();
        let hex = if c.starts_with('#') {
            c.strip_prefix('#').unwrap()
        } else {
            c
        };

        if hex.is_empty() {
            continue;
        }

        if hex.len() != 6 {
            return Err("Invalid color format, expected `#RRGGBB` or `RRGGBB`.".to_string());
        }

        if let Ok(rgb) = (0..6)
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
            .collect::<Result<Vec<_>, _>>()
        {
            colors.push(Rgb([rgb[0], rgb[1], rgb[2]]));
        } else {
            return Err(format!(
                "Invalid hexadecimal value contained in color: `{c}`"
            ));
        }
    }

    Ok(Box::from(colors))
}

fn generate_texture(colors: &[Rgb<u8>], size: u32) -> RgbImage {
    let len = (colors.len() as f32).sqrt().ceil() as u32;
    let mut image = RgbImage::new(len * size, len * size);

    for (i, c) in colors.iter().enumerate() {
        let start_pos = (i as u32 % len * size, (i as u32 / len) * size);
        for x in start_pos.0..start_pos.0 + size {
            for y in start_pos.1..start_pos.1 + size {
                image.put_pixel(x, y, *c);
            }
        }
    }

    image
}

fn main() {
    let args = Args::parse();

    generate_texture(&args.colors, args.size)
        .save(args.out)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_colors() {
        let input = "#FF5733,33FF57,#5733FF";
        let expected = Box::from([Rgb([255, 87, 51]), Rgb([51, 255, 87]), Rgb([87, 51, 255])]);

        let result = parse_colors(input).expect("Failed to parse colors");

        assert_eq!(result, expected);
    }
}
