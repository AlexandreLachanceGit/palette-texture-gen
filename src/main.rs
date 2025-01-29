use clap::{error::Result, Parser};
use image::Rgb;

/// Simple program to create a color swatch texture from a list of colors
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Hex Colors (comma separated)
    #[arg(required = true, value_parser = parse_colors, num_args = 1.., value_delimiter = ' ')]
    colors: Box<[Rgb<u8>]>,
}

fn parse_colors(s: &str) -> Result<Box<[Rgb<u8>]>, String> {
    let mut colors = vec![];
    for c in s.split(',') {
        let hex = if c.starts_with('#') {
            c.trim().strip_prefix('#').unwrap()
        } else {
            c.trim()
        };

        if hex.len() != 6 {
            return Err("Invalid color format, expected `#RRGGBB` or `RRGGBB`.".to_string());
        }

        if let Ok(rgb) = (0..6)
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
            .collect::<Result<Vec<_>, _>>()
        {
            println!("{rgb:?}");
            colors.push(Rgb([rgb[0], rgb[1], rgb[2]]));
        } else {
            return Err(format!(
                "Invalid hexadecimal value contained in color: `{c}`"
            ));
        }
    }

    Ok(Box::from(colors))
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.colors)
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
