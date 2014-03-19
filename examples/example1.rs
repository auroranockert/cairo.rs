#[feature(link_args)];

extern crate cairo;

#[link_args = "-L /Users/Jens/.homebrew/lib"]
extern {}

fn main() {
  use cairo;
  use cairo::matrix::Matrix;
  use cairo::surface;
  use cairo::surface::Surface;

  let (width, height) = (500.0, 500.0);
  let mut s = Surface::create_image(surface::format::ARGB32, width as i32, height as i32);

  let mut cairo = cairo::Cairo::create(&mut s);

  let m = Matrix::new(width, 1.0, 1.0, -height, 0.0, height);

  cairo.transform(&m);

  cairo.set_source_rgb(0.0, 0.0, 0.0);
  cairo.move_to(0.0, 0.0);
  cairo.line_to(1.0, 1.0);
  cairo.line_to(0.0, 1.0);
  cairo.set_line_width(0.2);
  cairo.stroke();
  cairo.fill();

  cairo.set_source_rgb(0.0, 0.0, 0.0);
  cairo.line_to(1.0, 1.0);
  cairo.move_to(1.0, 0.0);
  cairo.line_to(0.0, 1.0);
  cairo.set_line_width(0.2);
  cairo.stroke();

  cairo.rectangle(0.0, 0.0, 0.5, 0.5);
  cairo.set_source_rgba(1.0, 0.0, 0.0, 0.80);
  cairo.fill();

  cairo.rectangle(0.0, 0.5, 0.5, 0.5);
  cairo.set_source_rgba(0.0, 1.0, 0.0, 0.60);
  cairo.fill();

  cairo.rectangle(0.5, 0.0, 0.5, 0.5);
  cairo.set_source_rgba(0.0, 0.0, 1.0, 0.40);
  cairo.fill();

  s.write_to_png("example1.png");
  s.finish();
}