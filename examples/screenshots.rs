use screenshots::{Screen, PngImage};
use std::{fs, time::Instant};

fn main() {
  let start = Instant::now();
  let screens = Screen::all().unwrap();

  for screen in screens {
    println!("capturer {screen:?}");
    let mut image = screen.capture().unwrap();
    let mut png_image = PngImage::from_image(image).unwrap();
    fs::write(format!("target/{}.png", screen.display_info.id), png_image.bytes()).unwrap();

    image = screen.capture_area(300, 300, 300, 300).unwrap();
    png_image = PngImage::from_image(image).unwrap();
    fs::write(format!("target/{}-2.png", screen.display_info.id), png_image.bytes()).unwrap();
  }

  let screen = Screen::from_point(100, 100).unwrap();
  println!("capturer {screen:?}");

  let image = screen.capture_area(300, 300, 300, 300).unwrap();
  let png_image = PngImage::from_image(image).unwrap();
  fs::write("target/capture_display_with_point.png", png_image.bytes()).unwrap();

  println!("运行耗时: {:?}", start.elapsed());
}
