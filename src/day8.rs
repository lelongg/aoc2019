use anyhow::Result;
use std::str::FromStr;

type Pixel = u8;
const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const BLACK: Pixel = 0;
const WHITE: Pixel = 1;
const TRANSPARENT: Pixel = 2;

struct Layer {
    pixels: Vec<Pixel>,
}

impl Layer {
    fn count_value(&self, value: Pixel) -> usize {
        bytecount::count(&self.pixels, value)
    }
}

impl FromStr for Layer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            pixels: s
                .chars()
                .map(|digit| digit.to_digit(10).unwrap() as Pixel)
                .collect(),
        })
    }
}

struct Image {
    layers: Vec<Layer>,
}

impl Image {
    fn flatten_layers(&self) -> Layer {
        Layer {
            pixels: (0..WIDTH * HEIGHT)
                .map(|i| {
                    self.layers
                        .iter()
                        .map(|layer| layer.pixels[i])
                        .filter(|&pixel| pixel != TRANSPARENT)
                        .nth(0)
                        .unwrap_or(TRANSPARENT)
                })
                .collect(),
        }
    }

    fn render(&self) {
        let layer = self.flatten_layers();
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let pixel = layer.pixels[j * WIDTH + i % WIDTH];
                match pixel {
                    TRANSPARENT | BLACK => print!(" "),
                    WHITE => print!("█"),
                    _ => panic!("unknown pixel value"),
                }
            }
            println!();
        }
    }
}

impl FromStr for Image {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let chars = s.chars().collect::<Vec<_>>();
        Ok(Self {
            layers: chars
                .chunks(WIDTH * HEIGHT)
                .map(|chars| chars.iter().collect::<String>())
                .map(|s| Layer::from_str(&s))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day8.txt")?;
    let image = Image::from_str(input.trim())?;
    let min_zero_layer = image
        .layers
        .iter()
        .min_by_key(|layer| layer.count_value(0))
        .unwrap();
    let result = min_zero_layer.count_value(1) * min_zero_layer.count_value(2);
    println!("{}", result);
    image.render();
    Ok(())
}
