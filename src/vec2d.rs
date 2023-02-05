use std::f64::consts::PI;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2d {
  pub x: i64,
  pub y: i64,
}

impl Vec2d {
  pub fn add(&self, v: &Vec2d) -> Vec2d {
    Vec2d{
      x: self.x + v.x,
      y: self.y + v.y,
    }
  }

  pub fn subtract(&self, v: &Vec2d) -> Vec2d {
    Vec2d{
      x: self.x - v.x,
      y: self.y - v.y,
    }
  }

  pub fn mult(&self, s: i64) -> Vec2d {
    Vec2d{
      x: self.x * s,
      y: self.y * s,
    }
  }

  pub fn rotate(&self, degrees: i64) -> Vec2d {
    let r = (-degrees as f64) * PI / 180.;
    let cosr = r.cos();
    let sinr = r.sin();
    Vec2d {
      x: ((1e6 * (self.x as f64 * cosr - self.y as f64 * sinr)).round() / 1e6) as i64,
      y: ((1e6 * (self.x as f64 * sinr + self.y as f64 * cosr)).round() / 1e6) as i64,
    }
  }

  pub fn manhattan_distance(&self, v: &Vec2d) -> i64 {
    return (self.x - v.x).abs() + (self.y - v.y).abs();
  }
}
