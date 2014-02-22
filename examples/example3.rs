extern mod cairo;

fn main() {
  use cairo;
  use cairo::surface;
  use cairo::surface::Surface;

  let mut petal_size = 50.0;
  let size = petal_size * 8.0;

  let mut s = Surface::image(surface::format::ARGB32, size as i32, size as i32);
  let mut cairo = cairo::Cairo::new(&mut s);

  cairo.set_tolerance(0.1);

  /* Clear */
  cairo.set_operator(cairo::operator::Clear);
  cairo.paint();
  cairo.set_operator(cairo::operator::Over);

  cairo.translate(size / 2.0, size / 2.0);

  let n_groups = 3;
  for i in std::iter::range(0, n_groups) {
    let n_petals = [9, 7, 5][i];

    cairo.save();
    cairo.rotate([2.0, 1.0, 3.0][i]);

    match i {
      0 => cairo.set_source_rgba(1.00, 0.78, 0.57, 0.5),
      1 => cairo.set_source_rgba(0.91, 0.56, 0.64, 0.5),
      _ => cairo.set_source_rgba(0.51, 0.56, 0.67, 0.5)
    }

    let pm1 = [12.0, 16.0, 8.0][i];
    let pm2 = [3.0, 0.0, 1.0][i];

    for j in std::iter::range(1, n_petals + 1) {
      cairo.save();
      cairo.rotate(2.0 * (j as f64) * std::f64::consts::PI / (n_petals as f64));
      cairo.new_path();
      cairo.move_to(0.0, 0.0);
      cairo.rel_curve_to(petal_size, petal_size, (pm2 + 2.0) * petal_size, petal_size, 2.0 * petal_size + pm1, 0.0);
      cairo.rel_curve_to(pm2 * petal_size, -petal_size, -petal_size, -petal_size, -(2.0 * petal_size + pm1), 0.0);
      cairo.close_path();
      cairo.fill();
      cairo.restore();
    }

    petal_size -= [12.0, 4.0, 15.0][i];
    cairo.restore();
  }

  cairo.set_source_rgba(0.71, 0.81, 0.83, 0.5);

  cairo.arc(0.0, 0.0, petal_size, 0.0, 2.0 * std::f64::consts::PI);
  cairo.fill();

  s.to_png("example3.png");
  s.finish();
}