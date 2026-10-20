//! One continuous-corner silhouette for the lens, lighting, mask and DWM clip.
//! A superellipse of order 2.5 has zero curvature at its straight-edge joins.
//! Folding smoothly morphs the order toward circular capsule ends.
use windows_canvas::{GpuDevice, Path, PathBuilder, Vector2};
use windows_core::Result;

const SEGMENTS: usize = 64;

pub struct GlassShape {
    pub width: f32,
    pub height: f32,
    pub radius: f32,
    circular: bool,
    quarter: Vec<Vector2>,
    segments: Vec<(Vector2, Vector2, f32, Vector2)>,
    order: f32,
}

impl GlassShape {
    pub fn new(width: f32, height: f32, radius: f32) -> Self {
        let radius = radius.max(0.0).min(width.min(height) * 0.5);
        let circular = radius >= width.min(height) * 0.5 - 0.01;
        // Morph continuously into circular capsule ends as the two corners meet.
        // A binary order-2.5 -> circle switch visibly popped during a fold.
        let t = if radius > 0.0 {
            (width.min(height) / (2.0 * radius) - 1.0).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let order = 2.0 + 0.5 * t * t * (3.0 - 2.0 * t);
        let exponent = 2.0 / order;
        let quarter: Vec<_> = (0..=SEGMENTS)
            .map(|i| {
                let t = i as f32 / SEGMENTS as f32 * std::f32::consts::FRAC_PI_2;
                Vector2::new(
                    t.cos().max(0.0).powf(exponent),
                    t.sin().max(0.0).powf(exponent),
                )
            })
            .collect();
        let segments = quarter
            .windows(2)
            .map(|pair| {
                let a = pair[0];
                let v = pair[1] - a;
                let inverse_squared = 1.0 / (v.x * v.x + v.y * v.y);
                let normal = Vector2::new(v.y, -v.x) * inverse_squared.sqrt();
                (a, v, inverse_squared, normal)
            })
            .collect();
        Self {
            width,
            height,
            radius,
            circular,
            quarter,
            segments,
            order,
        }
    }

    /// Outward normal and signed inset in the same units as the shape.
    /// The corner distance is the nearest point on the actual clip polygon,
    /// rather than treating a p-norm as Euclidean distance.
    pub fn edge(&self, x: f32, y: f32) -> (f32, f32, f32) {
        let p = Vector2::new(x - self.width * 0.5, y - self.height * 0.5);
        let q = Vector2::new(
            p.x.abs() - self.width * 0.5 + self.radius,
            p.y.abs() - self.height * 0.5 + self.radius,
        );
        if self.radius <= 0.001 || q.x <= 0.0 || q.y <= 0.0 {
            return if q.x > q.y {
                (p.x.signum(), 0.0, self.radius - q.x)
            } else {
                (0.0, p.y.signum(), self.radius - q.y)
            };
        }
        if self.circular {
            let len = (q.x * q.x + q.y * q.y).sqrt();
            return (
                p.x.signum() * q.x / len,
                p.y.signum() * q.y / len,
                self.radius - len,
            );
        }
        let q = q / self.radius;
        let mut best = f32::INFINITY;
        let mut closest = Vector2::new(0.0, 0.0);
        let mut inside = true;
        for &(a, v, inverse_squared, n) in &self.segments {
            let t = (((q.x - a.x) * v.x + (q.y - a.y) * v.y) * inverse_squared).clamp(0.0, 1.0);
            let point = a + v * t;
            let delta = q - point;
            let distance = delta.x * delta.x + delta.y * delta.y;
            // The polygon is convex; an outside half-plane means outside the shape.
            inside &= (q.x - a.x) * n.x + (q.y - a.y) * n.y <= 0.0;
            if distance < best {
                best = distance;
                closest = point;
            }
        }
        // Calculate the smooth normal only at the final nearest point, rather than
        // repeatedly taking square roots for every candidate segment and every texel.
        let analytic = if self.order == 2.5 {
            Vector2::new(closest.x * closest.x.sqrt(), closest.y * closest.y.sqrt())
        } else {
            Vector2::new(
                closest.x.powf(self.order - 1.0),
                closest.y.powf(self.order - 1.0),
            )
        };
        let normal = analytic
            / (analytic.x * analytic.x + analytic.y * analytic.y)
                .sqrt()
                .max(1e-6);
        (
            normal.x * p.x.signum(),
            normal.y * p.y.signum(),
            best.sqrt() * self.radius * if inside { 1.0 } else { -1.0 },
        )
    }

    pub fn points(&self, top: f32) -> Vec<Vector2> {
        let r = self.radius;
        let centers = [
            Vector2::new(self.width - r, top + self.height - r),
            Vector2::new(r, top + self.height - r),
            Vector2::new(r, top + r),
            Vector2::new(self.width - r, top + r),
        ];
        let mut points = Vec::with_capacity((SEGMENTS + 1) * 4);
        for (quadrant, center) in centers.into_iter().enumerate() {
            for p in &self.quarter {
                let p = match quadrant {
                    0 => *p,
                    1 => Vector2::new(-p.y, p.x),
                    2 => Vector2::new(-p.x, -p.y),
                    _ => Vector2::new(p.y, -p.x),
                };
                points.push(center + p * r);
            }
        }
        points
    }

    /// Fast one-pixel coverage of the same polygon, for blurred CPU shadows.
    /// A radial binary search selects its boundary segment. The segment's plane
    /// gives subpixel coverage; no 64-segment nearest-normal search is needed.
    pub fn coverage(&self, x: f32, y: f32) -> f32 {
        let q = Vector2::new(
            (x - self.width * 0.5).abs() - self.width * 0.5 + self.radius,
            (y - self.height * 0.5).abs() - self.height * 0.5 + self.radius,
        );
        if self.circular || self.radius <= 0.001 || q.x <= 0.0 || q.y <= 0.0 {
            return (self.edge(x, y).2 + 0.5).clamp(0.0, 1.0);
        }
        let q = q / self.radius;
        let i = self
            .quarter
            .partition_point(|p| p.y * q.x < q.y * p.x)
            .saturating_sub(1)
            .min(SEGMENTS - 1);
        let (a, _, _, n) = self.segments[i];
        (0.5 - ((q.x - a.x) * n.x + (q.y - a.y) * n.y) * self.radius).clamp(0.0, 1.0)
    }

    pub fn path(&self, device: &GpuDevice, top: f32) -> Result<Path> {
        PathBuilder::new(device)?.polygon(self.points(top))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clip_and_optics_use_the_same_silhouette() {
        let shape = GlassShape::new(400.0, 260.0, 32.0);
        for p in shape.points(0.0) {
            assert!(shape.edge(p.x, p.y).2.abs() < 0.002);
        }
        assert!(shape.edge(8.0, 8.0).2 > 0.0);
        assert!(shape.edge(0.0, 0.0).2 < 0.0);
        assert_eq!(shape.edge(200.0, 100.0).2, 100.0);
    }
    #[test]
    fn corner_normal_joins_the_straight_edge_smoothly() {
        let shape = GlassShape::new(400.0, 260.0, 32.0);
        let a = shape.edge(31.9, 0.5);
        let b = shape.edge(32.1, 0.5);
        assert!((a.0 - b.0).abs() < 0.002 && (a.1 - b.1).abs() < 0.002);
        assert!((a.2 - b.2).abs() < 0.002);
        let capsule = GlassShape::new(120.0, 28.0, 14.0);
        assert!(capsule.circular);
        assert!(capsule.edge(4.0, 4.0).2 < 0.0);
    }

    #[test]
    fn fold_morph_and_fast_shadow_coverage_follow_the_clip() {
        let mut previous = None;
        for height in 80..=170 {
            let shape = GlassShape::new(400.0, height as f32, 42.0);
            let corner = shape.quarter[32] * shape.radius;
            if let Some(p) = previous {
                let delta: Vector2 = corner - p;
                assert!(
                    delta.x.abs() < 0.5 && delta.y.abs() < 0.5,
                    "corner popped at {height}"
                );
            }
            previous = Some(corner);
            for y in -2..48 {
                for x in -2..48 {
                    let x = x as f32 + 0.37;
                    let y = y as f32 + 0.61;
                    let exact = (shape.edge(x, y).2 + 0.5).clamp(0.0, 1.0);
                    assert!(
                        (shape.coverage(x, y) - exact).abs() < 0.004,
                        "shadow/clip mismatch at {height}, {x}, {y}"
                    );
                }
            }
        }
    }
}
