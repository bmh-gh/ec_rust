use std::{
  fmt::{self, Debug}
};

struct EllipticCurve {
  a: i128,
  b: i128,
  p: i128
}

impl EllipticCurve {
  fn new(a: i128, b: i128, p: i128) -> Self {
      Self { a, b, p }
  }

  fn contains(&self, point: &Point) -> bool {
      (point.y * point.y) % self.p == ((point.x * point.x * point.x) + (self.a * point.x) + self.b) % self.p
  }
}

impl fmt::Display for EllipticCurve {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "y^2 = x^3 + {}x + {} mod {}", self.a, self.b, self.p)
  }
}

// Elements of Z_p
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
  x: i128,
  y: i128
}

#[derive(Debug)]
struct InvalidPoint {
  not_on_point : String
}

impl InvalidPoint {
  fn new(point: Point, curve: EllipticCurve) -> Self {
      Self { not_on_point: format!("The Point ({}, {}) is not on the elliptic curve {}", point.x, point.y, curve) }
  }
}

impl fmt::Display for InvalidPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.not_on_point)
    }
}

impl Point {
  fn new(x: i128, y: i128) -> Self {
      Self { x, y }
  }

  fn add_over(one: Self, other: Self, curve: EllipticCurve) -> Result<Self, InvalidPoint> {
      // Add two points P and Q
      if !curve.contains(&one) {
          return Err(InvalidPoint::new(one, curve));
      }
      if !curve.contains(&other) {
          return Err(InvalidPoint::new(other, curve));
      }
      // Calculate slope of secant between points
      let s: i128;
      if one == other {
          // if P = Q then s is the derivative at the current point
          s = (((3 * one.x * one.x) + curve.a) * modular_inverse(2 * one.y, curve.p)) % curve.p;
      } else {
          // if P != Q then s is the slope of the secant
          s = ((other.y - one.y) * modular_inverse(other.x - one.x, curve.p)) % curve.p;
      }
      // Calculate x3
      let mut x3 = ((s * s) - one.x - other.x) % curve.p;
      println!("{}", x3);
      if x3 < 0 { x3 += curve.p; }
      // Calculate y3
      let mut y3 = (s * (one.x - x3) - one.y) % curve.p;
      if y3 < 0 { y3 += curve.p; }
      // R = (x3, y3)
      Ok(Self { x: x3, y: y3 })
  }
}

fn modular_inverse(a: i128, b: i128) -> i128 {
  let (mut s, mut old_s) = (0, 1);
  let (mut t, mut old_t) = (b, a);

  while t != 0 {
      let q = old_t / t;
      let (new_r, new_s) = (old_t - q * t, old_s - q * s);
      old_t = t;
      t = new_r;
      old_s = s;
      s = new_s;
  }

  if old_s < 0 {
      old_s = (old_s % b) + b;
  }

  return old_s;
}

#[cfg(test)]
mod tests {
  use crate::ec::{EllipticCurve, Point, modular_inverse};

  #[test]
  fn mod_inv() {
      // Test body:
      assert_eq!(modular_inverse(2, 17), 9);
  }

  #[test]
  fn point_addition() {
      let curve = EllipticCurve::new(2, 2, 17);
      let p = Point::new(5, 1);
      let r = Point::new(6, 3);
      assert_eq!(Point::add_over(p, p, curve).unwrap(), r)
  }

  #[test]
  fn point_on_function() {
      // Test body:
      let curve = EllipticCurve::new(2, 2, 17);
      let p = Point::new(5, 1);
      let q = Point::new(5, 2);
      assert!( curve.contains(&p) );
      assert!( !curve.contains(&q) );
  }
}
