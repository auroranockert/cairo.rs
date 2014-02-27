#[link(name = "cairo")]
extern {}

#[repr(i32)]
pub enum Status {
  Success = 0,
  NoMemory = 1,
  InvalidRestore = 2,
  InvalidPopGroup = 3,
  NoCurrentPoint = 4,
  InvalidMatrix = 5,
  InvalidStatus = 6,
  NullPointer = 7,
  InvalidString = 8,
  InvalidPathData = 9,
  ReadError = 10,
  WriteError = 11,
  SurfaceFinished = 12,
  TypeMismatch = 13,
  SurfaceTypeMismatch = 14,
  PatternTypeMismatch = 15,
  InvalidContent = 16,
  InvalidFormat = 17,
  InvalidVisual = 18,
  FileNotFound = 19,
  InvalidDash = 20,
  InvalidDSCComment = 21,
  InvalidIndex = 22,
  ClipNotRepresentable = 23,
  TempFileError = 24,
  InvalidStride = 25,
  FontTypeMismatch = 26,
  UserFontImmutable = 27,
  UserFontError = 28,
  NegativeCount = 29,
  InvalidClusters = 30,
  InvalidSlant = 31,
  InvalidWeight = 32,
  InvalidSize = 33,
  UserFontNotImplemented = 34,
  DeviceTypeMismatch = 35,
  DeviceError = 36,
  InvalidMeshConstruction = 37,
  DeviceFinished = 38
}

pub struct Cairo {
  opaque: *mut std::libc::c_void
}

impl Cairo {
  pub fn new(surface: &mut surface::Surface) -> Cairo {
    unsafe {
      let foreign_result = cairo_create(surface.opaque);
      return foreign_result;
    }
  }

  pub fn status(&mut self) -> Status {
    unsafe {
      let foreign_result = cairo_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn save(&mut self) {
    unsafe {
      cairo_save(self.opaque);
    }
  }

  pub fn restore(&mut self) {
    unsafe {
      cairo_restore(self.opaque);
    }
  }

  pub fn get_target(&mut self) -> surface::Surface {
    unsafe {
      let foreign_result = cairo_get_target(self.opaque);
      return foreign_result.clone();
    }
  }

  pub fn push_group(&mut self) {
    unsafe {
      cairo_push_group(self.opaque);
    }
  }

  pub fn push_group_with_content(&mut self, content: surface::content::Content) {
    unsafe {
      cairo_push_group_with_content(self.opaque, content);
    }
  }

  pub fn pop_group(&mut self) -> pattern::Pattern {
    unsafe {
      let foreign_result = cairo_pop_group(self.opaque);
      return foreign_result;
    }
  }

  pub fn pop_group_to_source(&mut self) {
    unsafe {
      cairo_pop_group_to_source(self.opaque);
    }
  }

  pub fn get_group_target(&mut self) -> surface::Surface {
    unsafe {
      let foreign_result = cairo_get_group_target(self.opaque);
      return foreign_result.clone();
    }
  }

  pub fn set_source_rgb(&mut self, red: f64, green: f64, blue: f64) {
    unsafe {
      cairo_set_source_rgb(self.opaque, red, green, blue);
    }
  }

  pub fn set_source_rgba(&mut self, red: f64, green: f64, blue: f64, alpha: f64) {
    unsafe {
      cairo_set_source_rgba(self.opaque, red, green, blue, alpha);
    }
  }

  pub fn set_source(&mut self, source: &mut pattern::Pattern) {
    unsafe {
      cairo_set_source(self.opaque, source.opaque);
    }
  }

  pub fn set_source_surface(&mut self, surface: &mut surface::Surface, x: f64, y: f64) {
    unsafe {
      cairo_set_source_surface(self.opaque, surface.opaque, x, y);
    }
  }

  pub fn get_source(&mut self) -> pattern::Pattern {
    unsafe {
      let foreign_result = cairo_get_source(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_antialias(&mut self, antialias: antialias::Antialias) {
    unsafe {
      cairo_set_antialias(self.opaque, antialias);
    }
  }

  pub fn get_antialias(&mut self) -> antialias::Antialias {
    unsafe {
      let foreign_result = cairo_get_antialias(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_dash(&mut self, dashes: &[f64], offset: f64) {
    unsafe {
      cairo_set_dash(self.opaque, dashes.as_ptr(), dashes.len() as i32, offset);
    }
  }

  pub fn get_dash_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_get_dash_count(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_dash(&mut self) -> (~[f64], f64) {
    unsafe {
      use std::num::Zero;
      use std::vec::MutableVector;
      let dashes_len = self.get_dash_count() as uint;
      let mut dashes:~[f64] = std::vec::from_elem(dashes_len, Zero::zero());
      let mut offset:f64 = std::unstable::intrinsics::init();
      cairo_get_dash(self.opaque, dashes.as_mut_ptr(), &mut offset);
      return (dashes, offset);
    }
  }

  pub fn set_fill_rule(&mut self, fill_rule: fill_rule::FillRule) {
    unsafe {
      cairo_set_fill_rule(self.opaque, fill_rule);
    }
  }

  pub fn get_fill_rule(&mut self) -> fill_rule::FillRule {
    unsafe {
      let foreign_result = cairo_get_fill_rule(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_line_cap(&mut self, line_cap: line_cap::LineCap) {
    unsafe {
      cairo_set_line_cap(self.opaque, line_cap);
    }
  }

  pub fn get_line_cap(&mut self) -> line_cap::LineCap {
    unsafe {
      let foreign_result = cairo_get_line_cap(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_line_join(&mut self, line_join: line_join::LineJoin) {
    unsafe {
      cairo_set_line_join(self.opaque, line_join);
    }
  }

  pub fn get_line_join(&mut self) -> line_join::LineJoin {
    unsafe {
      let foreign_result = cairo_get_line_join(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_line_width(&mut self, width: f64) {
    unsafe {
      cairo_set_line_width(self.opaque, width);
    }
  }

  pub fn get_line_width(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_line_width(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_miter_limit(&mut self, limit: f64) {
    unsafe {
      cairo_set_miter_limit(self.opaque, limit);
    }
  }

  pub fn get_miter_limit(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_miter_limit(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_operator(&mut self, operator: operator::Operator) {
    unsafe {
      cairo_set_operator(self.opaque, operator);
    }
  }

  pub fn get_operator(&mut self) -> operator::Operator {
    unsafe {
      let foreign_result = cairo_get_operator(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_tolerance(&mut self, tolerance: f64) {
    unsafe {
      cairo_set_tolerance(self.opaque, tolerance);
    }
  }

  pub fn get_tolerance(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_tolerance(self.opaque);
      return foreign_result;
    }
  }

  pub fn clip(&mut self) {
    unsafe {
      cairo_clip(self.opaque);
    }
  }

  pub fn clip_preserve(&mut self) {
    unsafe {
      cairo_clip_preserve(self.opaque);
    }
  }

  pub fn clip_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      let mut x1:f64 = std::unstable::intrinsics::init();
      let mut y1:f64 = std::unstable::intrinsics::init();
      let mut x2:f64 = std::unstable::intrinsics::init();
      let mut y2:f64 = std::unstable::intrinsics::init();
      cairo_clip_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  pub fn in_clip(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_clip(self.opaque, x, y);
      return foreign_result != 0;
    }
  }

  pub fn reset_clip(&mut self) {
    unsafe {
      cairo_reset_clip(self.opaque);
    }
  }

  pub fn fill(&mut self) {
    unsafe {
      cairo_fill(self.opaque);
    }
  }

  pub fn fill_preserve(&mut self) {
    unsafe {
      cairo_fill_preserve(self.opaque);
    }
  }

  pub fn fill_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      let mut x1:f64 = std::unstable::intrinsics::init();
      let mut y1:f64 = std::unstable::intrinsics::init();
      let mut x2:f64 = std::unstable::intrinsics::init();
      let mut y2:f64 = std::unstable::intrinsics::init();
      cairo_fill_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  pub fn in_fill(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_fill(self.opaque, x, y);
      return foreign_result != 0;
    }
  }

  pub fn mask(&mut self, pattern: &mut pattern::Pattern) {
    unsafe {
      cairo_mask(self.opaque, pattern.opaque);
    }
  }

  pub fn mask_surface(&mut self, surface: &mut surface::Surface, surface_x: f64, surface_y: f64) {
    unsafe {
      cairo_mask_surface(self.opaque, surface.opaque, surface_x, surface_y);
    }
  }

  pub fn paint(&mut self) {
    unsafe {
      cairo_paint(self.opaque);
    }
  }

  pub fn paint_with_alpha(&mut self, alpha: f64) {
    unsafe {
      cairo_paint_with_alpha(self.opaque, alpha);
    }
  }

  pub fn stroke(&mut self) {
    unsafe {
      cairo_stroke(self.opaque);
    }
  }

  pub fn stroke_preserve(&mut self) {
    unsafe {
      cairo_stroke_preserve(self.opaque);
    }
  }

  pub fn stroke_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      let mut x1:f64 = std::unstable::intrinsics::init();
      let mut y1:f64 = std::unstable::intrinsics::init();
      let mut x2:f64 = std::unstable::intrinsics::init();
      let mut y2:f64 = std::unstable::intrinsics::init();
      cairo_stroke_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  pub fn in_stroke(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_stroke(self.opaque, x, y);
      return foreign_result != 0;
    }
  }

  pub fn copy_page(&mut self) {
    unsafe {
      cairo_copy_page(self.opaque);
    }
  }

  pub fn show_page(&mut self) {
    unsafe {
      cairo_show_page(self.opaque);
    }
  }

  pub fn get_reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  pub fn copy_path(&mut self) -> path::Path {
    unsafe {
      let foreign_result = cairo_copy_path(self.opaque);
      return foreign_result;
    }
  }

  pub fn copy_path_flat(&mut self) -> path::Path {
    unsafe {
      let foreign_result = cairo_copy_path_flat(self.opaque);
      return foreign_result;
    }
  }

  pub fn append_path(&mut self, path: &path::Path) {
    unsafe {
      cairo_append_path(self.opaque, path.opaque as *std::libc::c_void);
    }
  }

  pub fn has_current_point(&mut self) -> bool {
    unsafe {
      let foreign_result = cairo_has_current_point(self.opaque);
      return foreign_result != 0;
    }
  }

  pub fn get_current_point(&mut self) -> (f64, f64) {
    unsafe {
      let mut x:f64 = std::unstable::intrinsics::init();
      let mut y:f64 = std::unstable::intrinsics::init();
      cairo_get_current_point(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }

  pub fn new_path(&mut self) {
    unsafe {
      cairo_new_path(self.opaque);
    }
  }

  pub fn new_sub_path(&mut self) {
    unsafe {
      cairo_new_sub_path(self.opaque);
    }
  }

  pub fn close_path(&mut self) {
    unsafe {
      cairo_close_path(self.opaque);
    }
  }

  pub fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
    unsafe {
      cairo_arc(self.opaque, xc, yc, radius, angle1, angle2);
    }
  }

  pub fn arc_negative(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
    unsafe {
      cairo_arc_negative(self.opaque, xc, yc, radius, angle1, angle2);
    }
  }

  pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    unsafe {
      cairo_curve_to(self.opaque, x1, y1, x2, y2, x3, y3);
    }
  }

  pub fn line_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_line_to(self.opaque, x, y);
    }
  }

  pub fn move_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_move_to(self.opaque, x, y);
    }
  }

  pub fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
    unsafe {
      cairo_rectangle(self.opaque, x, y, width, height);
    }
  }

  pub fn glyph_path(&mut self, glyphs: &[font::Glyph]) {
    unsafe {
      cairo_glyph_path(self.opaque, glyphs.as_ptr(), glyphs.len() as i32);
    }
  }

  pub fn text_path(&mut self, text_path: &str) {
    unsafe {
      use std::c_str::ToCStr;
      cairo_text_path(self.opaque, text_path.to_c_str().unwrap());
    }
  }

  pub fn rel_curve_to(&mut self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64) {
    unsafe {
      cairo_rel_curve_to(self.opaque, dx1, dy1, dx2, dy2, dx3, dy3);
    }
  }

  pub fn rel_line_to(&mut self, dx: f64, dy: f64) {
    unsafe {
      cairo_rel_line_to(self.opaque, dx, dy);
    }
  }

  pub fn rel_move_to(&mut self, dx: f64, dy: f64) {
    unsafe {
      cairo_rel_move_to(self.opaque, dx, dy);
    }
  }

  pub fn path_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      let mut x1:f64 = std::unstable::intrinsics::init();
      let mut y1:f64 = std::unstable::intrinsics::init();
      let mut x2:f64 = std::unstable::intrinsics::init();
      let mut y2:f64 = std::unstable::intrinsics::init();
      cairo_path_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  pub fn translate(&mut self, tx: f64, ty: f64) {
    unsafe {
      cairo_translate(self.opaque, tx, ty);
    }
  }

  pub fn scale(&mut self, sx: f64, sy: f64) {
    unsafe {
      cairo_scale(self.opaque, sx, sy);
    }
  }

  pub fn rotate(&mut self, angle: f64) {
    unsafe {
      cairo_rotate(self.opaque, angle);
    }
  }

  pub fn transform(&mut self, matrix: &matrix::Matrix) {
    unsafe {
      cairo_transform(self.opaque, matrix);
    }
  }

  pub fn set_matrix(&mut self, matrix: &matrix::Matrix) {
    unsafe {
      cairo_set_matrix(self.opaque, matrix);
    }
  }

  pub fn get_matrix(&mut self) -> matrix::Matrix {
    unsafe {
      let mut matrix:matrix::Matrix = std::unstable::intrinsics::init();
      cairo_get_matrix(self.opaque, &mut matrix);
      return matrix;
    }
  }

  pub fn identity_matrix(&mut self) {
    unsafe {
      cairo_identity_matrix(self.opaque);
    }
  }

  pub fn user_to_device(&mut self, x_r: f64, y_r: f64) -> (f64, f64) {
    unsafe {
      let mut x = x_r;
      let mut y = y_r;
      cairo_user_to_device(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }

  pub fn user_to_device_distance(&mut self, dx_r: f64, dy_r: f64) -> (f64, f64) {
    unsafe {
      let mut dx = dx_r;
      let mut dy = dy_r;
      cairo_user_to_device_distance(self.opaque, &mut dx, &mut dy);
      return (dx, dy);
    }
  }

  pub fn device_to_user(&mut self, x_r: f64, y_r: f64) -> (f64, f64) {
    unsafe {
      let mut x = x_r;
      let mut y = y_r;
      cairo_device_to_user(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }

  pub fn device_to_user_distance(&mut self, dx_r: f64, dy_r: f64) -> (f64, f64) {
    unsafe {
      let mut dx = dx_r;
      let mut dy = dy_r;
      cairo_device_to_user_distance(self.opaque, &mut dx, &mut dy);
      return (dx, dy);
    }
  }

  pub fn select_font_face(&mut self, family: &str, slant: font::slant::Slant, weight: font::weight::Weight) {
    unsafe {
      use std::c_str::ToCStr;
      cairo_select_font_face(self.opaque, family.to_c_str().unwrap(), slant, weight);
    }
  }

  pub fn set_font_size(&mut self, size: f64) {
    unsafe {
      cairo_set_font_size(self.opaque, size);
    }
  }

  pub fn set_font_matrix(&mut self, size: &matrix::Matrix) {
    unsafe {
      cairo_set_font_matrix(self.opaque, size);
    }
  }

  pub fn get_font_matrix(&mut self) -> matrix::Matrix {
    unsafe {
      let mut matrix:matrix::Matrix = std::unstable::intrinsics::init();
      cairo_get_font_matrix(self.opaque, &mut matrix);
      return matrix;
    }
  }

  pub fn set_font_options(&mut self, options: font::Options) {
    unsafe {
      cairo_set_font_options(self.opaque, options);
    }
  }

  pub fn get_font_options(&mut self, options: font::Options) {
    unsafe {
      cairo_get_font_options(self.opaque, options);
    }
  }

  pub fn set_font_face(&mut self, font_face: font::FontFace) {
    unsafe {
      cairo_set_font_face(self.opaque, font_face);
    }
  }

  pub fn get_font_face(&mut self) -> font::FontFace {
    unsafe {
      let foreign_result = cairo_get_font_face(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_scaled_font(&mut self, scaled_font: font::ScaledFont) {
    unsafe {
      cairo_set_scaled_font(self.opaque, scaled_font);
    }
  }

  pub fn get_scaled_font(&mut self) -> font::ScaledFont {
    unsafe {
      let foreign_result = cairo_get_scaled_font(self.opaque);
      return foreign_result;
    }
  }

  pub fn show_text(&mut self, utf8: &str) {
    unsafe {
      cairo_show_text(self.opaque, utf8);
    }
  }

  pub fn show_glyphs(&mut self, glyphs: &[font::Glyph]) {
    unsafe {
      cairo_show_glyphs(self.opaque, glyphs.as_ptr(), glyphs.len() as i32);
    }
  }

  pub fn show_text_glyphs(&mut self, utf8: &str, glyphs: &[font::Glyph], clusters: &[font::Cluster], cluster_flags: font::cluster_flags::ClusterFlags) {
    unsafe {
      cairo_show_text_glyphs(self.opaque, utf8, -1, glyphs.as_ptr(), glyphs.len() as i32, clusters.as_ptr(), clusters.len() as i32, cluster_flags);
    }
  }

  pub fn font_extents(&mut self) -> font::FontExtents {
    unsafe {
      let mut extents:font::FontExtents = std::unstable::intrinsics::init();
      cairo_font_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  pub fn text_extents(&mut self, utf8: &str) -> font::TextExtents {
    unsafe {
      let mut extents:font::TextExtents = std::unstable::intrinsics::init();
      cairo_text_extents(self.opaque, utf8, &mut extents);
      return extents;
    }
  }

  pub fn glyph_extents(&mut self, glyphs: &[font::Glyph]) -> font::TextExtents {
    unsafe {
      let mut extents:font::TextExtents = std::unstable::intrinsics::init();
      cairo_glyph_extents(self.opaque, glyphs.as_ptr(), glyphs.len() as i32, &mut extents);
      return extents;
    }
  }
}

extern {
  fn cairo_create(surface: *mut std::libc::c_void) -> Cairo;
  fn cairo_status(self_value: *mut std::libc::c_void) -> Status;
  fn cairo_save(self_value: *mut std::libc::c_void);
  fn cairo_restore(self_value: *mut std::libc::c_void);
  fn cairo_get_target(self_value: *mut std::libc::c_void) -> surface::Surface;
  fn cairo_push_group(self_value: *mut std::libc::c_void);
  fn cairo_push_group_with_content(self_value: *mut std::libc::c_void, content: surface::content::Content);
  fn cairo_pop_group(self_value: *mut std::libc::c_void) -> pattern::Pattern;
  fn cairo_pop_group_to_source(self_value: *mut std::libc::c_void);
  fn cairo_get_group_target(self_value: *mut std::libc::c_void) -> surface::Surface;
  fn cairo_set_source_rgb(self_value: *mut std::libc::c_void, red: f64, green: f64, blue: f64);
  fn cairo_set_source_rgba(self_value: *mut std::libc::c_void, red: f64, green: f64, blue: f64, alpha: f64);
  fn cairo_set_source(self_value: *mut std::libc::c_void, source: *mut std::libc::c_void);
  fn cairo_set_source_surface(self_value: *mut std::libc::c_void, surface: *mut std::libc::c_void, x: f64, y: f64);
  fn cairo_get_source(self_value: *mut std::libc::c_void) -> pattern::Pattern;
  fn cairo_set_antialias(self_value: *mut std::libc::c_void, antialias: antialias::Antialias);
  fn cairo_get_antialias(self_value: *mut std::libc::c_void) -> antialias::Antialias;
  fn cairo_set_dash(self_value: *mut std::libc::c_void, dashes: *f64, dashes_length: i32, offset: f64);
  fn cairo_get_dash_count(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_get_dash(self_value: *mut std::libc::c_void, dashes: *mut f64, offset: *mut f64);
  fn cairo_set_fill_rule(self_value: *mut std::libc::c_void, fill_rule: fill_rule::FillRule);
  fn cairo_get_fill_rule(self_value: *mut std::libc::c_void) -> fill_rule::FillRule;
  fn cairo_set_line_cap(self_value: *mut std::libc::c_void, line_cap: line_cap::LineCap);
  fn cairo_get_line_cap(self_value: *mut std::libc::c_void) -> line_cap::LineCap;
  fn cairo_set_line_join(self_value: *mut std::libc::c_void, line_join: line_join::LineJoin);
  fn cairo_get_line_join(self_value: *mut std::libc::c_void) -> line_join::LineJoin;
  fn cairo_set_line_width(self_value: *mut std::libc::c_void, width: f64);
  fn cairo_get_line_width(self_value: *mut std::libc::c_void) -> f64;
  fn cairo_set_miter_limit(self_value: *mut std::libc::c_void, limit: f64);
  fn cairo_get_miter_limit(self_value: *mut std::libc::c_void) -> f64;
  fn cairo_set_operator(self_value: *mut std::libc::c_void, operator: operator::Operator);
  fn cairo_get_operator(self_value: *mut std::libc::c_void) -> operator::Operator;
  fn cairo_set_tolerance(self_value: *mut std::libc::c_void, tolerance: f64);
  fn cairo_get_tolerance(self_value: *mut std::libc::c_void) -> f64;
  fn cairo_clip(self_value: *mut std::libc::c_void);
  fn cairo_clip_preserve(self_value: *mut std::libc::c_void);
  fn cairo_clip_extents(self_value: *mut std::libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_clip(self_value: *mut std::libc::c_void, x: f64, y: f64) -> i32;
  fn cairo_reset_clip(self_value: *mut std::libc::c_void);
  fn cairo_fill(self_value: *mut std::libc::c_void);
  fn cairo_fill_preserve(self_value: *mut std::libc::c_void);
  fn cairo_fill_extents(self_value: *mut std::libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_fill(self_value: *mut std::libc::c_void, x: f64, y: f64) -> i32;
  fn cairo_mask(self_value: *mut std::libc::c_void, pattern: *mut std::libc::c_void);
  fn cairo_mask_surface(self_value: *mut std::libc::c_void, surface: *mut std::libc::c_void, surface_x: f64, surface_y: f64);
  fn cairo_paint(self_value: *mut std::libc::c_void);
  fn cairo_paint_with_alpha(self_value: *mut std::libc::c_void, alpha: f64);
  fn cairo_stroke(self_value: *mut std::libc::c_void);
  fn cairo_stroke_preserve(self_value: *mut std::libc::c_void);
  fn cairo_stroke_extents(self_value: *mut std::libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_stroke(self_value: *mut std::libc::c_void, x: f64, y: f64) -> i32;
  fn cairo_copy_page(self_value: *mut std::libc::c_void);
  fn cairo_show_page(self_value: *mut std::libc::c_void);
  fn cairo_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_copy_path(self_value: *mut std::libc::c_void) -> path::Path;
  fn cairo_copy_path_flat(self_value: *mut std::libc::c_void) -> path::Path;
  fn cairo_append_path(self_value: *mut std::libc::c_void, path: *std::libc::c_void);
  fn cairo_has_current_point(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_get_current_point(self_value: *mut std::libc::c_void, x: *mut f64, y: *mut f64);
  fn cairo_new_path(self_value: *mut std::libc::c_void);
  fn cairo_new_sub_path(self_value: *mut std::libc::c_void);
  fn cairo_close_path(self_value: *mut std::libc::c_void);
  fn cairo_arc(self_value: *mut std::libc::c_void, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
  fn cairo_arc_negative(self_value: *mut std::libc::c_void, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
  fn cairo_curve_to(self_value: *mut std::libc::c_void, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
  fn cairo_line_to(self_value: *mut std::libc::c_void, x: f64, y: f64);
  fn cairo_move_to(self_value: *mut std::libc::c_void, x: f64, y: f64);
  fn cairo_rectangle(self_value: *mut std::libc::c_void, x: f64, y: f64, width: f64, height: f64);
  fn cairo_glyph_path(self_value: *mut std::libc::c_void, glyphs: *font::Glyph, glyphs_length: i32);
  fn cairo_text_path(self_value: *mut std::libc::c_void, text_path: *std::libc::c_char);
  fn cairo_rel_curve_to(self_value: *mut std::libc::c_void, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64);
  fn cairo_rel_line_to(self_value: *mut std::libc::c_void, dx: f64, dy: f64);
  fn cairo_rel_move_to(self_value: *mut std::libc::c_void, dx: f64, dy: f64);
  fn cairo_path_extents(self_value: *mut std::libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_translate(self_value: *mut std::libc::c_void, tx: f64, ty: f64);
  fn cairo_scale(self_value: *mut std::libc::c_void, sx: f64, sy: f64);
  fn cairo_rotate(self_value: *mut std::libc::c_void, angle: f64);
  fn cairo_transform(self_value: *mut std::libc::c_void, matrix: *matrix::Matrix);
  fn cairo_set_matrix(self_value: *mut std::libc::c_void, matrix: *matrix::Matrix);
  fn cairo_get_matrix(self_value: *mut std::libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_identity_matrix(self_value: *mut std::libc::c_void);
  fn cairo_user_to_device(self_value: *mut std::libc::c_void, x: &mut f64, y: &mut f64);
  fn cairo_user_to_device_distance(self_value: *mut std::libc::c_void, dx: &mut f64, dy: &mut f64);
  fn cairo_device_to_user(self_value: *mut std::libc::c_void, x: &mut f64, y: &mut f64);
  fn cairo_device_to_user_distance(self_value: *mut std::libc::c_void, dx: &mut f64, dy: &mut f64);
  fn cairo_select_font_face(self_value: *mut std::libc::c_void, family: *std::libc::c_char, slant: font::slant::Slant, weight: font::weight::Weight);
  fn cairo_set_font_size(self_value: *mut std::libc::c_void, size: f64);
  fn cairo_set_font_matrix(self_value: *mut std::libc::c_void, size: *matrix::Matrix);
  fn cairo_get_font_matrix(self_value: *mut std::libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_set_font_options(self_value: *mut std::libc::c_void, options: font::Options);
  fn cairo_get_font_options(self_value: *mut std::libc::c_void, options: font::Options);
  fn cairo_set_font_face(self_value: *mut std::libc::c_void, font_face: font::FontFace);
  fn cairo_get_font_face(self_value: *mut std::libc::c_void) -> font::FontFace;
  fn cairo_set_scaled_font(self_value: *mut std::libc::c_void, scaled_font: font::ScaledFont);
  fn cairo_get_scaled_font(self_value: *mut std::libc::c_void) -> font::ScaledFont;
  fn cairo_show_text(self_value: *mut std::libc::c_void, utf8: *str);
  fn cairo_show_glyphs(self_value: *mut std::libc::c_void, glyphs: *font::Glyph, glyphs_length: i32);
  fn cairo_show_text_glyphs(self_value: *mut std::libc::c_void, utf8: *str, utf8_len: i32, glyphs: *font::Glyph, glyphs_length: i32, clusters: *font::Cluster, clusters_length: i32, cluster_flags: font::cluster_flags::ClusterFlags);
  fn cairo_font_extents(self_value: *mut std::libc::c_void, extents: *mut font::FontExtents);
  fn cairo_text_extents(self_value: *mut std::libc::c_void, utf8: *str, extents: *mut font::TextExtents);
  fn cairo_glyph_extents(self_value: *mut std::libc::c_void, glyphs: *font::Glyph, glyphs_length: i32, extents: *mut font::TextExtents);
}

impl std::clone::Clone for Cairo {
  fn clone(&self) -> Cairo {
    unsafe {
      let foreign_result = cairo_reference(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_reference(self_value: *std::libc::c_void) -> Cairo;
}

impl std::ops::Drop for Cairo {
  fn drop(&mut self) {
    unsafe {
      cairo_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_destroy(self_value: *mut std::libc::c_void);
}

pub mod antialias {
  use std;

  #[repr(i32)]
  pub enum Antialias {
    Default = 0,
    None = 1,
    Gray = 2,
    Subpixel = 3,
    Fast = 4,
    Good = 5,
    Best = 6
  }
}

pub mod fill_rule {
  use std;

  #[repr(i32)]
  pub enum FillRule {
    Winding = 0,
    EvenOdd = 1
  }
}

pub mod line_cap {
  use std;

  #[repr(i32)]
  pub enum LineCap {
    Butt = 0,
    Round = 1,
    Square = 2
  }
}

pub mod line_join {
  use std;

  #[repr(i32)]
  pub enum LineJoin {
    Miter = 0,
    Round = 1,
    Bevel = 2
  }
}

pub mod operator {
  use std;

  #[repr(i32)]
  pub enum Operator {
    Clear = 0,
    Source = 1,
    Over = 2,
    In = 3,
    Out = 4,
    Atop = 5,
    Dest = 6,
    DestOver = 7,
    DestIn = 8,
    DestOut = 9,
    DestAtop = 10,
    Xor = 11,
    Add = 12,
    Saturate = 13,
    Multiply = 14,
    Screen = 15,
    Overlay = 16,
    Darken = 17,
    Lighten = 18,
    ColorDodge = 19,
    ColorBurn = 20,
    HardLight = 21,
    SoftLight = 22,
    Difference = 23,
    Exclusion = 24,
    HSLHue = 25,
    HSLSaturation = 26,
    HSLColor = 27,
    HSLLuminosity = 28
  }
}

pub mod path {
  use std;

  pub struct Path {
    opaque: *mut std::libc::c_void
  }

  impl std::ops::Drop for Path {
    fn drop(&mut self) {
      unsafe {
        cairo_path_destroy(self.opaque);
      }
    }
  }

  extern {
    fn cairo_path_destroy(self_value: *mut std::libc::c_void);
  }
}

pub mod pattern {
  use std;

  pub struct Pattern {
    opaque: *mut std::libc::c_void
  }

  impl Pattern {
    pub fn add_color_stop_rgb(&mut self, offset: f64, red: f64, green: f64, blue: f64) {
      unsafe {
        cairo_pattern_add_color_stop_rgb(self.opaque, offset, red, green, blue);
      }
    }

    pub fn add_color_stop_rgba(&mut self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
      unsafe {
        cairo_pattern_add_color_stop_rgba(self.opaque, offset, red, green, blue, alpha);
      }
    }

    pub fn get_color_stop_count(&mut self) -> (super::Status, i32) {
      unsafe {
        let mut stop_count:i32 = std::unstable::intrinsics::init();
        let foreign_result = cairo_pattern_get_color_stop_count(self.opaque, &mut stop_count);
        return (foreign_result, stop_count);
      }
    }

    pub fn get_color_stop_rgba(&mut self, stop_count: i32) -> (super::Status, f64, f64, f64, f64, f64) {
      unsafe {
        let mut offset:f64 = std::unstable::intrinsics::init();
        let mut red:f64 = std::unstable::intrinsics::init();
        let mut green:f64 = std::unstable::intrinsics::init();
        let mut blue:f64 = std::unstable::intrinsics::init();
        let mut alpha:f64 = std::unstable::intrinsics::init();
        let foreign_result = cairo_pattern_get_color_stop_rgba(self.opaque, stop_count, &mut offset, &mut red, &mut green, &mut blue, &mut alpha);
        return (foreign_result, offset, red, green, blue, alpha);
      }
    }

    pub fn rgb(red: f64, green: f64, blue: f64) -> Pattern {
      unsafe {
        let foreign_result = cairo_pattern_create_rgb(red, green, blue);
        return foreign_result;
      }
    }

    pub fn rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Pattern {
      unsafe {
        let foreign_result = cairo_pattern_create_rgba(red, green, blue, alpha);
        return foreign_result;
      }
    }

    pub fn get_rgba(&mut self) -> (super::Status, f64, f64, f64, f64) {
      unsafe {
        let mut red:f64 = std::unstable::intrinsics::init();
        let mut green:f64 = std::unstable::intrinsics::init();
        let mut blue:f64 = std::unstable::intrinsics::init();
        let mut alpha:f64 = std::unstable::intrinsics::init();
        let foreign_result = cairo_pattern_get_rgba(self.opaque, &mut red, &mut green, &mut blue, &mut alpha);
        return (foreign_result, red, green, blue, alpha);
      }
    }

    pub fn for_surface(surface: super::surface::Surface) -> Pattern {
      unsafe {
        let foreign_result = cairo_pattern_create_for_surface(surface);
        return foreign_result;
      }
    }

    pub fn get_surface(&mut self) -> (super::Status, super::surface::Surface) {
      unsafe {
        let mut surface:super::surface::Surface = std::unstable::intrinsics::init();
        let foreign_result = cairo_pattern_get_surface(self.opaque, &mut surface);
        return (foreign_result, surface);
      }
    }

    pub fn linear(x0: f64, y0: f64, x1: f64, y1: f64) -> Pattern {
      unsafe {
        let foreign_result = cairo_pattern_create_linear(x0, y0, x1, y1);
        return foreign_result;
      }
    }

    pub fn get_linear_points(&mut self) -> (super::Status, f64, f64, f64, f64) {
      unsafe {
        let mut x0:f64 = std::unstable::intrinsics::init();
        let mut y0:f64 = std::unstable::intrinsics::init();
        let mut x1:f64 = std::unstable::intrinsics::init();
        let mut y1:f64 = std::unstable::intrinsics::init();
        let foreign_result = cairo_pattern_get_linear_points(self.opaque, &mut x0, &mut y0, &mut x1, &mut y1);
        return (foreign_result, x0, y0, x1, y1);
      }
    }

    pub fn radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> Pattern {
      unsafe {
        let foreign_result = cairo_pattern_create_radial(cx0, cy0, radius0, cx1, cy1, radius1);
        return foreign_result;
      }
    }

    pub fn get_radial_circles(&mut self) -> (super::Status, f64, f64, f64, f64, f64, f64) {
      unsafe {
        let mut x0:f64 = std::unstable::intrinsics::init();
        let mut y0:f64 = std::unstable::intrinsics::init();
        let mut r0:f64 = std::unstable::intrinsics::init();
        let mut x1:f64 = std::unstable::intrinsics::init();
        let mut y1:f64 = std::unstable::intrinsics::init();
        let mut r1:f64 = std::unstable::intrinsics::init();
        let foreign_result = cairo_pattern_get_radial_circles(self.opaque, &mut x0, &mut y0, &mut r0, &mut x1, &mut y1, &mut r1);
        return (foreign_result, x0, y0, r0, x1, y1, r1);
      }
    }

    pub fn mesh() -> Pattern {
      unsafe {
        let foreign_result = cairo_pattern_create_mesh();
        return foreign_result;
      }
    }

    pub fn begin_patch(&mut self) {
      unsafe {
        cairo_mesh_pattern_begin_patch(self.opaque);
      }
    }

    pub fn end_patch(&mut self) {
      unsafe {
        cairo_mesh_pattern_end_patch(self.opaque);
      }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
      unsafe {
        cairo_mesh_pattern_move_to(self.opaque, x, y);
      }
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
      unsafe {
        cairo_mesh_pattern_line_to(self.opaque, x, y);
      }
    }

    pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
      unsafe {
        cairo_mesh_pattern_curve_to(self.opaque, x1, y1, x2, y2, x3, y3);
      }
    }

    pub fn set_control_point(&mut self, point_num: i32, x: f64, y: f64) {
      unsafe {
        cairo_mesh_pattern_set_control_point(self.opaque, point_num, x, y);
      }
    }

    pub fn set_corner_color_rgb(&mut self, corner_num: i32, red: f64, green: f64, blue: f64) {
      unsafe {
        cairo_mesh_pattern_set_corner_color_rgb(self.opaque, corner_num, red, green, blue);
      }
    }

    pub fn set_corner_color_rgba(&mut self, corner_num: i32, red: f64, green: f64, blue: f64, alpha: f64) {
      unsafe {
        cairo_mesh_pattern_set_corner_color_rgba(self.opaque, corner_num, red, green, blue, alpha);
      }
    }

    pub fn get_patch_count(&mut self) -> (super::Status, i32) {
      unsafe {
        let mut count:i32 = std::unstable::intrinsics::init();
        let foreign_result = cairo_mesh_pattern_get_patch_count(self.opaque, &mut count);
        return (foreign_result, count);
      }
    }

    pub fn get_path(&mut self, patch_num: i32) -> super::path::Path {
      unsafe {
        let foreign_result = cairo_mesh_pattern_get_path(self.opaque, patch_num);
        return foreign_result;
      }
    }

    pub fn get_control_point(&mut self, patch_num: i32, pointer_num: i32) -> (super::Status, f64, f64) {
      unsafe {
        let mut x:f64 = std::unstable::intrinsics::init();
        let mut y:f64 = std::unstable::intrinsics::init();
        let foreign_result = cairo_mesh_pattern_get_control_point(self.opaque, patch_num, pointer_num, &mut x, &mut y);
        return (foreign_result, x, y);
      }
    }

    pub fn get_corner_color_rgba(&mut self, patch_num: i32, pointer_num: i32) -> (super::Status, f64, f64, f64, f64) {
      unsafe {
        let mut red:f64 = std::unstable::intrinsics::init();
        let mut green:f64 = std::unstable::intrinsics::init();
        let mut blue:f64 = std::unstable::intrinsics::init();
        let mut alpha:f64 = std::unstable::intrinsics::init();
        let foreign_result = cairo_mesh_pattern_get_corner_color_rgba(self.opaque, patch_num, pointer_num, &mut red, &mut green, &mut blue, &mut alpha);
        return (foreign_result, red, green, blue, alpha);
      }
    }

    pub fn status(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_pattern_status(self.opaque);
        return foreign_result;
      }
    }

    pub fn set_extend(&mut self, extend: extend::Extend) {
      unsafe {
        cairo_pattern_set_extend(self.opaque, extend);
      }
    }

    pub fn get_extend(&mut self) -> extend::Extend {
      unsafe {
        let foreign_result = cairo_pattern_get_extend(self.opaque);
        return foreign_result;
      }
    }

    pub fn set_filter(&mut self, filter: filter::Filter) {
      unsafe {
        cairo_pattern_set_filter(self.opaque, filter);
      }
    }

    pub fn get_filter(&mut self) -> filter::Filter {
      unsafe {
        let foreign_result = cairo_pattern_get_filter(self.opaque);
        return foreign_result;
      }
    }

    pub fn set_matrix(&mut self, matrix: super::matrix::Matrix) {
      unsafe {
        cairo_pattern_set_matrix(self.opaque, matrix);
      }
    }

    pub fn get_matrix(&mut self) -> super::matrix::Matrix {
      unsafe {
        let foreign_result = cairo_pattern_get_matrix(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_type(&mut self) -> pattern_type::PatternType {
      unsafe {
        let foreign_result = cairo_pattern_get_type(self.opaque);
        return foreign_result;
      }
    }

    pub fn reference_count(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_pattern_get_reference_count(self.opaque);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_pattern_add_color_stop_rgb(self_value: *mut std::libc::c_void, offset: f64, red: f64, green: f64, blue: f64);
    fn cairo_pattern_add_color_stop_rgba(self_value: *mut std::libc::c_void, offset: f64, red: f64, green: f64, blue: f64, alpha: f64);
    fn cairo_pattern_get_color_stop_count(self_value: *mut std::libc::c_void, stop_count: *mut i32) -> super::Status;
    fn cairo_pattern_get_color_stop_rgba(self_value: *mut std::libc::c_void, stop_count: i32, offset: *mut f64, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
    fn cairo_pattern_create_rgb(red: f64, green: f64, blue: f64) -> Pattern;
    fn cairo_pattern_create_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Pattern;
    fn cairo_pattern_get_rgba(self_value: *mut std::libc::c_void, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
    fn cairo_pattern_create_for_surface(surface: super::surface::Surface) -> Pattern;
    fn cairo_pattern_get_surface(self_value: *mut std::libc::c_void, surface: *mut super::surface::Surface) -> super::Status;
    fn cairo_pattern_create_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> Pattern;
    fn cairo_pattern_get_linear_points(self_value: *mut std::libc::c_void, x0: *mut f64, y0: *mut f64, x1: *mut f64, y1: *mut f64) -> super::Status;
    fn cairo_pattern_create_radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> Pattern;
    fn cairo_pattern_get_radial_circles(self_value: *mut std::libc::c_void, x0: *mut f64, y0: *mut f64, r0: *mut f64, x1: *mut f64, y1: *mut f64, r1: *mut f64) -> super::Status;
    fn cairo_pattern_create_mesh() -> Pattern;
    fn cairo_mesh_pattern_begin_patch(self_value: *mut std::libc::c_void);
    fn cairo_mesh_pattern_end_patch(self_value: *mut std::libc::c_void);
    fn cairo_mesh_pattern_move_to(self_value: *mut std::libc::c_void, x: f64, y: f64);
    fn cairo_mesh_pattern_line_to(self_value: *mut std::libc::c_void, x: f64, y: f64);
    fn cairo_mesh_pattern_curve_to(self_value: *mut std::libc::c_void, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
    fn cairo_mesh_pattern_set_control_point(self_value: *mut std::libc::c_void, point_num: i32, x: f64, y: f64);
    fn cairo_mesh_pattern_set_corner_color_rgb(self_value: *mut std::libc::c_void, corner_num: i32, red: f64, green: f64, blue: f64);
    fn cairo_mesh_pattern_set_corner_color_rgba(self_value: *mut std::libc::c_void, corner_num: i32, red: f64, green: f64, blue: f64, alpha: f64);
    fn cairo_mesh_pattern_get_patch_count(self_value: *mut std::libc::c_void, count: *mut i32) -> super::Status;
    fn cairo_mesh_pattern_get_path(self_value: *mut std::libc::c_void, patch_num: i32) -> super::path::Path;
    fn cairo_mesh_pattern_get_control_point(self_value: *mut std::libc::c_void, patch_num: i32, pointer_num: i32, x: *mut f64, y: *mut f64) -> super::Status;
    fn cairo_mesh_pattern_get_corner_color_rgba(self_value: *mut std::libc::c_void, patch_num: i32, pointer_num: i32, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
    fn cairo_pattern_status(self_value: *mut std::libc::c_void) -> super::Status;
    fn cairo_pattern_set_extend(self_value: *mut std::libc::c_void, extend: extend::Extend);
    fn cairo_pattern_get_extend(self_value: *mut std::libc::c_void) -> extend::Extend;
    fn cairo_pattern_set_filter(self_value: *mut std::libc::c_void, filter: filter::Filter);
    fn cairo_pattern_get_filter(self_value: *mut std::libc::c_void) -> filter::Filter;
    fn cairo_pattern_set_matrix(self_value: *mut std::libc::c_void, matrix: super::matrix::Matrix);
    fn cairo_pattern_get_matrix(self_value: *mut std::libc::c_void) -> super::matrix::Matrix;
    fn cairo_pattern_get_type(self_value: *mut std::libc::c_void) -> pattern_type::PatternType;
    fn cairo_pattern_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
  }

  impl std::clone::Clone for Pattern {
    fn clone(&self) -> Pattern {
      unsafe {
        let foreign_result = cairo_pattern_reference(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_pattern_reference(self_value: *std::libc::c_void) -> Pattern;
  }

  impl std::ops::Drop for Pattern {
    fn drop(&mut self) {
      unsafe {
        cairo_pattern_destroy(self.opaque);
      }
    }
  }

  extern {
    fn cairo_pattern_destroy(self_value: *mut std::libc::c_void);
  }

  pub mod extend {
    use std;

    #[repr(i32)]
    pub enum Extend {
      None = 0,
      Repeat = 1,
      Reflect = 2
    }
  }

  pub mod filter {
    use std;

    #[repr(i32)]
    pub enum Filter {
      Fast = 0,
      Good = 1,
      Best = 2,
      Nearest = 3,
      Bilinear = 4,
      Gaussian = 5
    }
  }

  pub mod pattern_type {
    use std;

    #[repr(i32)]
    pub enum PatternType {
      Solid = 0,
      Surface = 1,
      Linear = 2,
      Radial = 3,
      Mesh = 4,
      RasterSource = 5
    }
  }
}

pub mod region {
  use std;

  pub struct Region {
    opaque: *mut std::libc::c_void
  }

  pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32
  }

  impl Region {
    pub fn new() -> Region {
      unsafe {
        let foreign_result = cairo_region_create();
        return foreign_result;
      }
    }

    pub fn rectangle(rectangle: &Rectangle) -> Region {
      unsafe {
        let foreign_result = cairo_region_create_rectangle(rectangle);
        return foreign_result;
      }
    }

    pub fn rectangles(rectangles: &[Rectangle]) -> Region {
      unsafe {
        let foreign_result = cairo_region_create_rectangles(rectangles.as_ptr(), rectangles.len() as i32);
        return foreign_result;
      }
    }

    pub fn status(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_region_status(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_extents(&mut self) -> Rectangle {
      unsafe {
        let mut extents:Rectangle = std::unstable::intrinsics::init();
        cairo_region_get_extents(self.opaque, &mut extents);
        return extents;
      }
    }

    pub fn num_rectangles(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_region_num_rectangles(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_rectangle(&mut self, nth: i32) -> Rectangle {
      unsafe {
        let mut rectangle:Rectangle = std::unstable::intrinsics::init();
        cairo_region_get_rectangle(self.opaque, nth, &mut rectangle);
        return rectangle;
      }
    }

    pub fn is_empty(&mut self) -> bool {
      unsafe {
        let foreign_result = cairo_region_is_empty(self.opaque);
        return foreign_result != 0;
      }
    }

    pub fn contains_point(&mut self, x: i32, y: i32) -> bool {
      unsafe {
        let foreign_result = cairo_region_contains_point(self.opaque, x, y);
        return foreign_result != 0;
      }
    }

    pub fn containts_rectangle(&mut self, rectangle: &Rectangle) -> overlap::Overlap {
      unsafe {
        let foreign_result = cairo_region_contains_rectangle(self.opaque, rectangle);
        return foreign_result;
      }
    }

    pub fn equal(&mut self, other: &Region) -> bool {
      unsafe {
        let foreign_result = cairo_region_equal(self.opaque, other.opaque as *std::libc::c_void);
        return foreign_result != 0;
      }
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
      unsafe {
        cairo_region_translate(self.opaque, dx, dy);
      }
    }

    pub fn intersect_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
      unsafe {
        let foreign_result = cairo_region_intersect_rectangle(self.opaque, rectangle);
        return foreign_result;
      }
    }

    pub fn subtract(&mut self, region: &Region) -> super::Status {
      unsafe {
        let foreign_result = cairo_region_subtract(self.opaque, region);
        return foreign_result;
      }
    }

    pub fn subtract_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
      unsafe {
        let foreign_result = cairo_region_subtract_rectangle(self.opaque, rectangle);
        return foreign_result;
      }
    }

    pub fn union(&mut self, region: &Region) -> super::Status {
      unsafe {
        let foreign_result = cairo_region_union(self.opaque, region);
        return foreign_result;
      }
    }

    pub fn union_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
      unsafe {
        let foreign_result = cairo_region_union_rectangle(self.opaque, rectangle);
        return foreign_result;
      }
    }

    pub fn xor(&mut self, region: &Region) -> super::Status {
      unsafe {
        let foreign_result = cairo_region_xor(self.opaque, region);
        return foreign_result;
      }
    }

    pub fn xor_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
      unsafe {
        let foreign_result = cairo_region_xor_rectangle(self.opaque, rectangle);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_region_create() -> Region;
    fn cairo_region_create_rectangle(rectangle: *Rectangle) -> Region;
    fn cairo_region_create_rectangles(rectangles: *Rectangle, rectangles_length: i32) -> Region;
    fn cairo_region_status(self_value: *mut std::libc::c_void) -> super::Status;
    fn cairo_region_get_extents(self_value: *mut std::libc::c_void, extents: *mut Rectangle);
    fn cairo_region_num_rectangles(self_value: *mut std::libc::c_void) -> i32;
    fn cairo_region_get_rectangle(self_value: *mut std::libc::c_void, nth: i32, rectangle: *mut Rectangle);
    fn cairo_region_is_empty(self_value: *mut std::libc::c_void) -> i32;
    fn cairo_region_contains_point(self_value: *mut std::libc::c_void, x: i32, y: i32) -> i32;
    fn cairo_region_contains_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> overlap::Overlap;
    fn cairo_region_equal(self_value: *mut std::libc::c_void, other: *std::libc::c_void) -> i32;
    fn cairo_region_translate(self_value: *mut std::libc::c_void, dx: i32, dy: i32);
    fn cairo_region_intersect_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> super::Status;
    fn cairo_region_subtract(self_value: *mut std::libc::c_void, region: *Region) -> super::Status;
    fn cairo_region_subtract_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> super::Status;
    fn cairo_region_union(self_value: *mut std::libc::c_void, region: *Region) -> super::Status;
    fn cairo_region_union_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> super::Status;
    fn cairo_region_xor(self_value: *mut std::libc::c_void, region: *Region) -> super::Status;
    fn cairo_region_xor_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> super::Status;
  }

  impl std::clone::Clone for Region {
    fn clone(&self) -> Region {
      unsafe {
        let foreign_result = cairo_region_reference(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_region_reference(self_value: *std::libc::c_void) -> Region;
  }

  impl std::clone::DeepClone for Region {
    fn deep_clone(&self) -> Region {
      unsafe {
        let foreign_result = cairo_region_copy(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_region_copy(self_value: *std::libc::c_void) -> Region;
  }

  impl std::ops::Drop for Region {
    fn drop(&mut self) {
      unsafe {
        cairo_region_destroy(self.opaque);
      }
    }
  }

  extern {
    fn cairo_region_destroy(self_value: *mut std::libc::c_void);
  }

  pub mod overlap {
    use std;

    #[repr(i32)]
    pub enum Overlap {
      In = 0,
      Out = 1,
      Part = 2
    }
  }
}

pub mod font {
  use std;

  pub struct Options {
    opaque: *mut std::libc::c_void
  }

  pub struct FontFace {
    opaque: *mut std::libc::c_void
  }

  pub struct ScaledFont {
    opaque: *mut std::libc::c_void
  }

  pub struct Glyph {
    index: i64,
    x: f64,
    y: f64
  }

  pub struct Cluster {
    num_bytes: i32,
    num_glyphs: i32
  }

  pub struct FontExtents {
    ascent: f64,
    descent: f64,
    height: f64,
    max_x_advance: f64,
    max_y_advance: f64
  }

  pub struct TextExtents {
    x_bearing: f64,
    y_bearing: f64,
    width: f64,
    height: f64,
    max_x_advance: f64,
    max_y_advance: f64
  }

  impl Options {
    pub fn new() -> Options {
      unsafe {
        let foreign_result = cairo_font_options_create();
        return foreign_result;
      }
    }

    pub fn status(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_font_options_status(self.opaque);
        return foreign_result;
      }
    }

    pub fn merge(&mut self, other: &Options) {
      unsafe {
        cairo_font_options_merge(self.opaque, other);
      }
    }

    pub fn hash(&mut self) -> i64 {
      unsafe {
        let foreign_result = cairo_font_options_hash(self.opaque);
        return foreign_result;
      }
    }

    pub fn equal(&mut self, other: &Options) -> bool {
      unsafe {
        let foreign_result = cairo_font_options_equal(self.opaque, other.opaque as *std::libc::c_void);
        return foreign_result != 0;
      }
    }

    pub fn set_antialias(&mut self, antialias: super::antialias::Antialias) {
      unsafe {
        cairo_font_options_set_antialias(self.opaque, antialias);
      }
    }

    pub fn get_antialias(&mut self) -> super::antialias::Antialias {
      unsafe {
        let foreign_result = cairo_font_options_get_antialias(self.opaque);
        return foreign_result;
      }
    }

    pub fn set_subpixel_order(&mut self, subpixel_order: subpixel_order::SubpixelOrder) {
      unsafe {
        cairo_font_options_set_subpixel_order(self.opaque, subpixel_order);
      }
    }

    pub fn get_subpixel_order(&mut self) -> subpixel_order::SubpixelOrder {
      unsafe {
        let foreign_result = cairo_font_options_get_subpixel_order(self.opaque);
        return foreign_result;
      }
    }

    pub fn set_hint_style(&mut self, hint_style: hint_style::HintStyle) {
      unsafe {
        cairo_font_options_set_hint_style(self.opaque, hint_style);
      }
    }

    pub fn get_hint_style(&mut self) -> hint_style::HintStyle {
      unsafe {
        let foreign_result = cairo_font_options_get_hint_style(self.opaque);
        return foreign_result;
      }
    }

    pub fn set_hint_metrics(&mut self, hint_metrics: hint_metrics::HintMetrics) {
      unsafe {
        cairo_font_options_set_hint_metrics(self.opaque, hint_metrics);
      }
    }

    pub fn get_hint_metrics(&mut self) -> hint_metrics::HintMetrics {
      unsafe {
        let foreign_result = cairo_font_options_get_hint_metrics(self.opaque);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_font_options_create() -> Options;
    fn cairo_font_options_status(self_value: *mut std::libc::c_void) -> super::Status;
    fn cairo_font_options_merge(self_value: *mut std::libc::c_void, other: *Options);
    fn cairo_font_options_hash(self_value: *mut std::libc::c_void) -> i64;
    fn cairo_font_options_equal(self_value: *mut std::libc::c_void, other: *std::libc::c_void) -> i32;
    fn cairo_font_options_set_antialias(self_value: *mut std::libc::c_void, antialias: super::antialias::Antialias);
    fn cairo_font_options_get_antialias(self_value: *mut std::libc::c_void) -> super::antialias::Antialias;
    fn cairo_font_options_set_subpixel_order(self_value: *mut std::libc::c_void, subpixel_order: subpixel_order::SubpixelOrder);
    fn cairo_font_options_get_subpixel_order(self_value: *mut std::libc::c_void) -> subpixel_order::SubpixelOrder;
    fn cairo_font_options_set_hint_style(self_value: *mut std::libc::c_void, hint_style: hint_style::HintStyle);
    fn cairo_font_options_get_hint_style(self_value: *mut std::libc::c_void) -> hint_style::HintStyle;
    fn cairo_font_options_set_hint_metrics(self_value: *mut std::libc::c_void, hint_metrics: hint_metrics::HintMetrics);
    fn cairo_font_options_get_hint_metrics(self_value: *mut std::libc::c_void) -> hint_metrics::HintMetrics;
  }

  impl std::clone::Clone for Options {
    fn clone(&self) -> Options {
      unsafe {
        let foreign_result = cairo_font_options_copy(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_font_options_copy(self_value: *std::libc::c_void) -> Options;
  }

  impl std::clone::DeepClone for Options {
    fn deep_clone(&self) -> Options {
      unsafe {
        let foreign_result = cairo_font_options_copy(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
  }

  impl std::ops::Drop for Options {
    fn drop(&mut self) {
      unsafe {
        cairo_font_options_destroy(self.opaque);
      }
    }
  }

  extern {
    fn cairo_font_options_destroy(self_value: *mut std::libc::c_void);
  }

  impl FontFace {
    pub fn toy(family: &str, slant: slant::Slant, weight: weight::Weight) -> FontFace {
      unsafe {
        use std::c_str::ToCStr;
        let foreign_result = cairo_toy_font_face_create(family.to_c_str().unwrap(), slant, weight);
        return foreign_result;
      }
    }

    pub fn toy_get_family(&mut self) -> std::c_str::CString {
      unsafe {
        let foreign_result = cairo_toy_font_face_get_family(self.opaque);
        return std::c_str::CString::new(foreign_result, false);
      }
    }

    pub fn toy_get_slant(&mut self) -> slant::Slant {
      unsafe {
        let foreign_result = cairo_toy_font_face_get_slant(self.opaque);
        return foreign_result;
      }
    }

    pub fn toy_get_weight(&mut self) -> slant::Slant {
      unsafe {
        let foreign_result = cairo_toy_font_face_get_weight(self.opaque);
        return foreign_result;
      }
    }

    pub fn status(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_font_face_status(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_type(&mut self) -> font_type::FontType {
      unsafe {
        let foreign_result = cairo_font_face_get_type(self.opaque);
        return foreign_result;
      }
    }

    pub fn reference_count(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_font_face_get_reference_count(self.opaque);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_toy_font_face_create(family: *std::libc::c_char, slant: slant::Slant, weight: weight::Weight) -> FontFace;
    fn cairo_toy_font_face_get_family(self_value: *mut std::libc::c_void) -> *i8;
    fn cairo_toy_font_face_get_slant(self_value: *mut std::libc::c_void) -> slant::Slant;
    fn cairo_toy_font_face_get_weight(self_value: *mut std::libc::c_void) -> slant::Slant;
    fn cairo_font_face_status(self_value: *mut std::libc::c_void) -> super::Status;
    fn cairo_font_face_get_type(self_value: *mut std::libc::c_void) -> font_type::FontType;
    fn cairo_font_face_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
  }

  impl std::clone::Clone for FontFace {
    fn clone(&self) -> FontFace {
      unsafe {
        let foreign_result = cairo_font_face_reference(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_font_face_reference(self_value: *std::libc::c_void) -> FontFace;
  }

  impl std::ops::Drop for FontFace {
    fn drop(&mut self) {
      unsafe {
        cairo_font_face_destroy(self.opaque);
      }
    }
  }

  extern {
    fn cairo_font_face_destroy(self_value: *mut std::libc::c_void);
  }

  impl ScaledFont {
    pub fn new(font_face: &mut FontFace, font_matrix: &super::matrix::Matrix, ctm: &super::matrix::Matrix, options: &mut Options) -> ScaledFont {
      unsafe {
        let foreign_result = cairo_scaled_font_create(font_face, font_matrix, ctm, options);
        return foreign_result;
      }
    }

    pub fn status(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_scaled_font_status(self.opaque);
        return foreign_result;
      }
    }

    pub fn font_extents(&mut self) -> FontExtents {
      unsafe {
        let mut extents:FontExtents = std::unstable::intrinsics::init();
        cairo_scaled_font_extents(self.opaque, &mut extents);
        return extents;
      }
    }

    pub fn text_extents(&mut self, utf8: &str) -> TextExtents {
      unsafe {
        let mut extents:TextExtents = std::unstable::intrinsics::init();
        cairo_scaled_font_text_extents(self.opaque, utf8, &mut extents);
        return extents;
      }
    }

    pub fn glyph_extents(&mut self, glyphs: &[Glyph]) -> TextExtents {
      unsafe {
        let mut extents:TextExtents = std::unstable::intrinsics::init();
        cairo_scaled_font_glyph_extents(self.opaque, glyphs.as_ptr(), glyphs.len() as i32, &mut extents);
        return extents;
      }
    }

    pub fn get_font_face(&mut self) -> FontFace {
      unsafe {
        let foreign_result = cairo_scaled_font_get_font_face(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_font_options(&mut self, options: FontExtents) {
      unsafe {
        cairo_scaled_font_get_font_options(self.opaque, options);
      }
    }

    pub fn get_font_matrix(&mut self) -> super::matrix::Matrix {
      unsafe {
        let mut font_matrix:super::matrix::Matrix = std::unstable::intrinsics::init();
        cairo_scaled_font_get_font_matrix(self.opaque, &mut font_matrix);
        return font_matrix;
      }
    }

    pub fn get_ctm(&mut self) -> super::matrix::Matrix {
      unsafe {
        let mut ctm:super::matrix::Matrix = std::unstable::intrinsics::init();
        cairo_scaled_font_get_ctm(self.opaque, &mut ctm);
        return ctm;
      }
    }

    pub fn get_scale_matrix(&mut self) -> super::matrix::Matrix {
      unsafe {
        let mut scale_matrix:super::matrix::Matrix = std::unstable::intrinsics::init();
        cairo_scaled_font_get_scale_matrix(self.opaque, &mut scale_matrix);
        return scale_matrix;
      }
    }

    pub fn get_type(&mut self) -> font_type::FontType {
      unsafe {
        let foreign_result = cairo_scaled_font_get_type(self.opaque);
        return foreign_result;
      }
    }

    pub fn reference_count(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_scaled_font_get_reference_count(self.opaque);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_scaled_font_create(font_face: *mut FontFace, font_matrix: *super::matrix::Matrix, ctm: *super::matrix::Matrix, options: *mut Options) -> ScaledFont;
    fn cairo_scaled_font_status(self_value: *mut std::libc::c_void) -> super::Status;
    fn cairo_scaled_font_extents(self_value: *mut std::libc::c_void, extents: *mut FontExtents);
    fn cairo_scaled_font_text_extents(self_value: *mut std::libc::c_void, utf8: *str, extents: *mut TextExtents);
    fn cairo_scaled_font_glyph_extents(self_value: *mut std::libc::c_void, glyphs: *Glyph, glyphs_length: i32, extents: *mut TextExtents);
    fn cairo_scaled_font_get_font_face(self_value: *mut std::libc::c_void) -> FontFace;
    fn cairo_scaled_font_get_font_options(self_value: *mut std::libc::c_void, options: FontExtents);
    fn cairo_scaled_font_get_font_matrix(self_value: *mut std::libc::c_void, font_matrix: *mut super::matrix::Matrix);
    fn cairo_scaled_font_get_ctm(self_value: *mut std::libc::c_void, ctm: *mut super::matrix::Matrix);
    fn cairo_scaled_font_get_scale_matrix(self_value: *mut std::libc::c_void, scale_matrix: *mut super::matrix::Matrix);
    fn cairo_scaled_font_get_type(self_value: *mut std::libc::c_void) -> font_type::FontType;
    fn cairo_scaled_font_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
  }

  impl std::clone::Clone for ScaledFont {
    fn clone(&self) -> ScaledFont {
      unsafe {
        let foreign_result = cairo_scaled_font_reference(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_scaled_font_reference(self_value: *std::libc::c_void) -> ScaledFont;
  }

  impl std::ops::Drop for ScaledFont {
    fn drop(&mut self) {
      unsafe {
        cairo_scaled_font_destroy(self.opaque);
      }
    }
  }

  extern {
    fn cairo_scaled_font_destroy(self_value: *mut std::libc::c_void);
  }

  pub mod cluster_flags {
    use std;

    #[repr(i32)]
    pub enum ClusterFlags {
      Forwards = 0,
      Backwards = 1
    }
  }

  pub mod font_type {
    use std;

    #[repr(i32)]
    pub enum FontType {
      Toy = 0,
      FT = 1,
      Win32 = 2,
      Quartz = 3,
      User = 4
    }
  }

  pub mod slant {
    use std;

    #[repr(i32)]
    pub enum Slant {
      Normal = 0,
      Italic = 1,
      Oblique = 2
    }
  }

  pub mod weight {
    use std;

    #[repr(i32)]
    pub enum Weight {
      Normal = 0,
      Bold = 1
    }
  }

  pub mod subpixel_order {
    use std;

    #[repr(i32)]
    pub enum SubpixelOrder {
      Default = 0,
      RGB = 1,
      BGR = 2,
      VRGB = 3,
      VBGR = 4
    }
  }

  pub mod hint_style {
    use std;

    #[repr(i32)]
    pub enum HintStyle {
      Default = 0,
      None = 1,
      Slight = 2,
      Medium = 3,
      Full = 4
    }
  }

  pub mod hint_metrics {
    use std;

    #[repr(i32)]
    pub enum HintMetrics {
      Default = 0,
      Off = 1,
      On = 2
    }
  }
}

pub mod surface {
  use std;

  #[repr(i32)]
  pub enum SVGVersion {
    SVGVersion_1_1 = 0,
    SVGVersion_1_2 = 1
  }

  pub struct Surface {
    opaque: *mut std::libc::c_void
  }

  pub struct Device {
    opaque: *mut std::libc::c_void
  }

  impl Device {
    pub fn status(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_device_status(self.opaque);
        return foreign_result;
      }
    }

    pub fn finish(&mut self) {
      unsafe {
        cairo_device_finish(self.opaque);
      }
    }

    pub fn flush(&mut self) {
      unsafe {
        cairo_device_flush(self.opaque);
      }
    }

    pub fn get_type(&mut self) -> device_type::DeviceType {
      unsafe {
        let foreign_result = cairo_device_get_type(self.opaque);
        return foreign_result;
      }
    }

    pub fn reference_type(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_device_get_reference_count(self.opaque);
        return foreign_result;
      }
    }

    pub fn acquire(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_device_acquire(self.opaque);
        return foreign_result;
      }
    }

    pub fn release(&mut self) {
      unsafe {
        cairo_device_release(self.opaque);
      }
    }
  }

  extern {
    fn cairo_device_status(self_value: *mut std::libc::c_void) -> super::Status;
    fn cairo_device_finish(self_value: *mut std::libc::c_void);
    fn cairo_device_flush(self_value: *mut std::libc::c_void);
    fn cairo_device_get_type(self_value: *mut std::libc::c_void) -> device_type::DeviceType;
    fn cairo_device_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
    fn cairo_device_acquire(self_value: *mut std::libc::c_void) -> super::Status;
    fn cairo_device_release(self_value: *mut std::libc::c_void);
  }

  impl std::clone::Clone for Device {
    fn clone(&self) -> Device {
      unsafe {
        let foreign_result = cairo_device_reference(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_device_reference(self_value: *std::libc::c_void) -> Device;
  }

  impl std::ops::Drop for Device {
    fn drop(&mut self) {
      unsafe {
        cairo_device_destroy(self.opaque);
      }
    }
  }

  extern {
    fn cairo_device_destroy(self_value: *mut std::libc::c_void);
  }

  impl Surface {
    pub fn similar_image(format: format::Format, width: i32, height: i32) -> Surface {
      unsafe {
        let foreign_result = cairo_surface_create_similar_image(format, width, height);
        return foreign_result;
      }
    }

    pub fn for_rectangle(x: f64, y: f64, width: f64, height: f64) -> Surface {
      unsafe {
        let foreign_result = cairo_surface_create_for_rectangle(x, y, width, height);
        return foreign_result;
      }
    }

    pub fn status(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_surface_status(self.opaque);
        return foreign_result;
      }
    }

    pub fn finish(&mut self) {
      unsafe {
        cairo_surface_finish(self.opaque);
      }
    }

    pub fn flush(&mut self) {
      unsafe {
        cairo_surface_flush(self.opaque);
      }
    }

    pub fn get_device(&mut self) -> Device {
      unsafe {
        let foreign_result = cairo_surface_get_device(self.opaque);
        return foreign_result.clone();
      }
    }

    pub fn get_font_options(&mut self, options: &mut super::font::Options) {
      unsafe {
        cairo_surface_get_font_options(self.opaque, options.opaque);
      }
    }

    pub fn get_content(&mut self) -> content::Content {
      unsafe {
        let foreign_result = cairo_surface_get_content(self.opaque);
        return foreign_result;
      }
    }

    pub fn mark_dirty(&mut self) {
      unsafe {
        cairo_surface_mark_dirty(self.opaque);
      }
    }

    pub fn mark_dirty_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
      unsafe {
        cairo_surface_mark_dirty_rectangle(self.opaque, x, y, width, height);
      }
    }

    pub fn set_device_offset(&mut self, x_offset: f64, y_offset: f64) {
      unsafe {
        cairo_surface_set_device_offset(self.opaque, x_offset, y_offset);
      }
    }

    pub fn get_device_offset(&mut self) -> (f64, f64) {
      unsafe {
        let mut x_offset:f64 = std::unstable::intrinsics::init();
        let mut y_offset:f64 = std::unstable::intrinsics::init();
        cairo_surface_get_device_offset(self.opaque, &mut x_offset, &mut y_offset);
        return (x_offset, y_offset);
      }
    }

    pub fn surface_set_fallback_resolution(&mut self, x_pixels_per_inch: f64, y_pixels_per_inch: f64) {
      unsafe {
        cairo_surface_set_fallback_resolution(self.opaque, x_pixels_per_inch, y_pixels_per_inch);
      }
    }

    pub fn get_fallback_resolution(&mut self) -> (f64, f64) {
      unsafe {
        let mut x_pixels_per_inch:f64 = std::unstable::intrinsics::init();
        let mut y_pixels_per_inch:f64 = std::unstable::intrinsics::init();
        cairo_surface_get_fallback_resolution(self.opaque, &mut x_pixels_per_inch, &mut y_pixels_per_inch);
        return (x_pixels_per_inch, y_pixels_per_inch);
      }
    }

    pub fn get_type(&mut self) -> surface_type::SurfaceType {
      unsafe {
        let foreign_result = cairo_surface_get_type(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_reference_count(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_surface_get_reference_count(self.opaque);
        return foreign_result;
      }
    }

    pub fn copy_page(&mut self) {
      unsafe {
        cairo_surface_copy_page(self.opaque);
      }
    }

    pub fn show_page(&mut self) {
      unsafe {
        cairo_surface_show_page(self.opaque);
      }
    }

    pub fn image(format: format::Format, width: i32, height: i32) -> Surface {
      unsafe {
        let foreign_result = cairo_image_surface_create(format, width, height);
        return foreign_result;
      }
    }

    pub fn get_format(&mut self) -> format::Format {
      unsafe {
        let foreign_result = cairo_image_surface_get_format(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_width(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_image_surface_get_width(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_height(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_image_surface_get_height(self.opaque);
        return foreign_result;
      }
    }

    pub fn get_stride(&mut self) -> i32 {
      unsafe {
        let foreign_result = cairo_image_surface_get_stride(self.opaque);
        return foreign_result;
      }
    }

    pub fn png(filename: &str) -> Surface {
      unsafe {
        use std::c_str::ToCStr;
        let foreign_result = cairo_image_surface_create_from_png(filename.to_c_str().unwrap());
        return foreign_result;
      }
    }

    pub fn to_png(&mut self, filename: &str) -> super::Status {
      unsafe {
        use std::c_str::ToCStr;
        let foreign_result = cairo_surface_write_to_png(self.opaque, filename.to_c_str().unwrap());
        return foreign_result;
      }
    }

    pub fn svg(filename: &str, width: f64, height: f64) -> Surface {
      unsafe {
        use std::c_str::ToCStr;
        let foreign_result = cairo_svg_surface_create(filename.to_c_str().unwrap(), width, height);
        return foreign_result;
      }
    }

    pub fn restrict_to_svg_version(&mut self, version: SVGVersion) {
      unsafe {
        cairo_svg_surface_restrict_to_version(self, version);
      }
    }

    pub fn svg_version_to_string(version: SVGVersion) -> std::c_str::CString {
      unsafe {
        let foreign_result = cairo_svg_version_to_string(version);
        return std::c_str::CString::new(foreign_result, false);
      }
    }
  }

  extern {
    fn cairo_surface_create_similar_image(format: format::Format, width: i32, height: i32) -> Surface;
    fn cairo_surface_create_for_rectangle(x: f64, y: f64, width: f64, height: f64) -> Surface;
    fn cairo_surface_status(self_value: *mut std::libc::c_void) -> super::Status;
    fn cairo_surface_finish(self_value: *mut std::libc::c_void);
    fn cairo_surface_flush(self_value: *mut std::libc::c_void);
    fn cairo_surface_get_device(self_value: *mut std::libc::c_void) -> Device;
    fn cairo_surface_get_font_options(self_value: *mut std::libc::c_void, options: *mut std::libc::c_void);
    fn cairo_surface_get_content(self_value: *mut std::libc::c_void) -> content::Content;
    fn cairo_surface_mark_dirty(self_value: *mut std::libc::c_void);
    fn cairo_surface_mark_dirty_rectangle(self_value: *mut std::libc::c_void, x: f64, y: f64, width: f64, height: f64);
    fn cairo_surface_set_device_offset(self_value: *mut std::libc::c_void, x_offset: f64, y_offset: f64);
    fn cairo_surface_get_device_offset(self_value: *mut std::libc::c_void, x_offset: *mut f64, y_offset: *mut f64);
    fn cairo_surface_set_fallback_resolution(self_value: *mut std::libc::c_void, x_pixels_per_inch: f64, y_pixels_per_inch: f64);
    fn cairo_surface_get_fallback_resolution(self_value: *mut std::libc::c_void, x_pixels_per_inch: *mut f64, y_pixels_per_inch: *mut f64);
    fn cairo_surface_get_type(self_value: *mut std::libc::c_void) -> surface_type::SurfaceType;
    fn cairo_surface_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
    fn cairo_surface_copy_page(self_value: *mut std::libc::c_void);
    fn cairo_surface_show_page(self_value: *mut std::libc::c_void);
    fn cairo_image_surface_create(format: format::Format, width: i32, height: i32) -> Surface;
    fn cairo_image_surface_get_format(self_value: *mut std::libc::c_void) -> format::Format;
    fn cairo_image_surface_get_width(self_value: *mut std::libc::c_void) -> i32;
    fn cairo_image_surface_get_height(self_value: *mut std::libc::c_void) -> i32;
    fn cairo_image_surface_get_stride(self_value: *mut std::libc::c_void) -> i32;
    fn cairo_image_surface_create_from_png(filename: *std::libc::c_char) -> Surface;
    fn cairo_surface_write_to_png(self_value: *mut std::libc::c_void, filename: *std::libc::c_char) -> super::Status;
    fn cairo_svg_surface_create(filename: *std::libc::c_char, width: f64, height: f64) -> Surface;
    fn cairo_svg_surface_restrict_to_version(self_value: *mut Surface, version: SVGVersion);
    fn cairo_svg_version_to_string(version: SVGVersion) -> *i8;
  }

  impl std::clone::Clone for Surface {
    fn clone(&self) -> Surface {
      unsafe {
        let foreign_result = cairo_surface_reference(self.opaque as *std::libc::c_void);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_surface_reference(self_value: *std::libc::c_void) -> Surface;
  }

  impl std::ops::Drop for Surface {
    fn drop(&mut self) {
      unsafe {
        cairo_surface_destroy(self.opaque);
      }
    }
  }

  extern {
    fn cairo_surface_destroy(self_value: *mut std::libc::c_void);
  }

  pub mod content {
    use std;

    #[repr(i32)]
    pub enum Content {
      Color = 0x1000,
      Alpha = 0x2000,
      ColorAlpha = 0x3000
    }
  }

  pub mod device_type {
    use std;

    #[repr(i32)]
    pub enum DeviceType {
      Invalid = -1,
      DRM = 0,
      GL = 1,
      Script = 2,
      XCB = 3,
      XLib = 4,
      XML = 5,
      COGL = 6,
      Win32 = 7
    }
  }

  pub mod surface_type {
    use std;

    #[repr(i32)]
    pub enum SurfaceType {
      Image = 0,
      PDF = 1,
      PS = 2,
      XLib = 3,
      Glitz = 4,
      Quartz = 5,
      Win32 = 6,
      BeOS = 7,
      DirectFB = 8,
      SVG = 9,
      OS2 = 10,
      Win32Printing = 11,
      QuartzImage = 12,
      Script = 13,
      Qt = 14,
      Recording = 15,
      VG = 16,
      GL = 17,
      DRM = 18,
      Tee = 19,
      XML = 20,
      Skia = 21,
      Subsurface = 22,
      CoGL = 23
    }
  }

  pub mod format {
    use std;

    #[repr(i32)]
    pub enum Format {
      Invalid = -1,
      ARGB32 = 0,
      RGB24 = 1,
      A8 = 2,
      A1 = 3,
      RGB16_565 = 4,
      RGB30 = 5
    }
  }
}

pub mod matrix {
  use std;

  pub struct Matrix {
    xx: f64,
    yx: f64,
    xy: f64,
    yy: f64,
    x0: f64,
    y0: f64
  }

  impl Matrix {
    pub fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Matrix {
      unsafe {
        let mut this:Matrix = std::unstable::intrinsics::init();
        cairo_matrix_init(&mut this, xx, yx, xy, yy, x0, y0);
        return this;
      }
    }

    pub fn identity() -> Matrix {
      unsafe {
        let mut this:Matrix = std::unstable::intrinsics::init();
        cairo_matrix_init_identity(&mut this);
        return this;
      }
    }

    pub fn for_translation(x0: f64, y0: f64) -> Matrix {
      unsafe {
        let mut this:Matrix = std::unstable::intrinsics::init();
        cairo_matrix_init_translate(&mut this, x0, y0);
        return this;
      }
    }

    pub fn for_scale(sx: f64, sy: f64) -> Matrix {
      unsafe {
        let mut this:Matrix = std::unstable::intrinsics::init();
        cairo_matrix_init_scale(&mut this, sx, sy);
        return this;
      }
    }

    pub fn for_rotation(radians: f64) -> Matrix {
      unsafe {
        let mut this:Matrix = std::unstable::intrinsics::init();
        cairo_matrix_init_rotate(&mut this, radians);
        return this;
      }
    }

    pub fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
      unsafe {
        let mut this:Matrix = std::unstable::intrinsics::init();
        cairo_matrix_multiply(&mut this, a, b);
        return this;
      }
    }

    pub fn translate(&mut self, x0: f64, y0: f64) {
      unsafe {
        cairo_matrix_translate(self, x0, y0);
      }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
      unsafe {
        cairo_matrix_scale(self, sx, sy);
      }
    }

    pub fn rotate(&mut self, radians: f64) {
      unsafe {
        cairo_matrix_rotate(self, radians);
      }
    }

    pub fn transform_distance(&self, dx_r: f64, dy_r: f64) -> (f64, f64) {
      unsafe {
        let mut dx = dx_r;
        let mut dy = dy_r;
        cairo_matrix_transform_distance(self, &mut dx, &mut dy);
        return (dx, dy);
      }
    }

    pub fn transform_point(&self, x_r: f64, y_r: f64) -> (f64, f64) {
      unsafe {
        let mut x = x_r;
        let mut y = y_r;
        cairo_matrix_transform_point(self, &mut x, &mut y);
        return (x, y);
      }
    }

    pub fn invert(&mut self) -> super::Status {
      unsafe {
        let foreign_result = cairo_matrix_invert(self);
        return foreign_result;
      }
    }
  }

  extern {
    fn cairo_matrix_init(this: *mut Matrix, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);
    fn cairo_matrix_init_identity(this: *mut Matrix);
    fn cairo_matrix_init_translate(this: *mut Matrix, x0: f64, y0: f64);
    fn cairo_matrix_init_scale(this: *mut Matrix, sx: f64, sy: f64);
    fn cairo_matrix_init_rotate(this: *mut Matrix, radians: f64);
    fn cairo_matrix_multiply(this: *mut Matrix, a: *Matrix, b: *Matrix);
    fn cairo_matrix_translate(self_value: *mut Matrix, x0: f64, y0: f64);
    fn cairo_matrix_scale(self_value: *mut Matrix, sx: f64, sy: f64);
    fn cairo_matrix_rotate(self_value: *mut Matrix, radians: f64);
    fn cairo_matrix_transform_distance(self_value: *Matrix, dx: &mut f64, dy: &mut f64);
    fn cairo_matrix_transform_point(self_value: *Matrix, x: &mut f64, y: &mut f64);
    fn cairo_matrix_invert(self_value: *mut Matrix) -> super::Status;
  }
}
