fn get_n_diverged(x0: f64, y0: f64, max_iter: usize) -> u8 {
  // 複素数z_nの実部をxn,虚部をynとします
  let mut xn = 0.0
  let mut yn = 0.0;
  for i in 1..max_iter {
    let x_next = xn * xn - yn * yn + x0;
    let y_next = 2.0 * xn * yn + y0;
    xn = x_next;
    yn = y_next;
    if yn * yn + xn * xn > 4.0 {
      return i as u8;
    }
  }
  max_iter as u8
}

pub fn generate_mandelbrot_set(
  canvas_w: usize,
  canvas_h: usize,
  x_min: f64,
  x_max: f64,
  y_min: f64,
  y_max: f64,
  max_iter: usize
) -> Vec<u8> {
  let canvas_w_f64 = canvas_w as f64;
  let canvas_h_f64 = canvas_h as f64;
  let mut data = vec![];
  for i in 0..canvas_h {
    let y = y_min + (y_max - y_min) * i as f64 / canvas_h_f64;
    for j in 0..canvas_w {
      let x = x_min + (x_max - x_min) * j as f64 / canvas_w_f64;
      let iter_index = get_n_diverged(x, y, max_iter);
      let v = iter_index % 8 * 32;
      data.push(v);
      data.push(v);
      data.push(v);
      data.push(255);
    }
  }
  data
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_get_n_diverged() {
    let max_iter = 10;
    assert_eq!(get_n_diverged(1.0, 0.0, max_iter), 3);
    assert_eq!(get_n_diverged(0.0, 0.0, max_iter), max_iter as u8);
    assert_eq!(get_n_diverged(0.0, 1.0, max_iter), max_iter as u8);
  }
}