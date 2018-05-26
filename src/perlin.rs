// PerlinNoise gen taken from https://github.com/processing-js/processing-js/blob/master/src/P5Functions/Math.js

use rand::prelude::*;

/// Perlin Noise generator that outputs 1/2/3D Perlin noise
#[derive(Clone)]
pub struct PerlinNoise {
    perm: [usize; 512],
    octaves: usize,
    fallout: f64,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut perm = [0; 512];
        let mut rng = thread_rng();

        for i in 0..256 {
            perm[i] = i;
        }

        for i in 0..256 {
            let j = rng.gen_range(0, 256) & 0xFF;
            let t = perm[j];

            perm[j] = perm[i];
            perm[i] = t;
        }

        for i in 0..256 {
            perm[i + 256] = perm[i];
        }

        PerlinNoise {
            perm,
            octaves: 4,
            fallout: 0.5,
        }
    }

    /// Perlin Noise in 3D
    pub fn get3d(&self, args: [f64; 3]) -> f64 {
        let mut effect = 1.0;
        let mut k = 1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * (1.0 + self.noise3d(k * args[0], k * args[1], k * args[2])) / 2.0;
            k *= 2.0
        }

        sum
    }
    /// Perlin Noise in 2D
    pub fn get2d(&self, args: [f64; 2]) -> f64 {
        let mut effect = 1.0;
        let mut k = 1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * ((1.0 + self.noise2d(k * args[0], k * args[1])) / 2.0);

            k *= 2.0
        }

        sum
    }

    /// Perlin Noise in 1D
    pub fn get(&self, x: f64) -> f64 {
        let mut effect = 1.0;
        let mut k = 1.0;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            effect *= self.fallout;
            sum += effect * ((1.0 + self.noise1d(k * x)) / 2.0);
            k *= 2.0
        }

        sum
    }

    fn noise3d(&self, mut x: f64, mut y: f64, mut z: f64) -> f64 {
        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let z0 = z.floor() as usize;

        x -= x.floor();
        y -= y.floor();
        z -= z.floor();

        let fx = (3.0 - 2.0 * x) * x * x;
        let fy = (3.0 - 2.0 * y) * y * y;
        let fz = (3.0 - 2.0 * z) * z * z;

        let p0 = self.perm[x0] + y0;
        let p00 = self.perm[p0] + z0;
        let p01 = self.perm[p0 + 1] + z0;
        let p1 = self.perm[x0 + 1] + y0;
        let p10 = self.perm[p1] + z0;
        let p11 = self.perm[p1 + 1] + z0;

        lerp(
            fz,
            lerp(
                fy,
                lerp(
                    fx,
                    grad3d(self.perm[p00], x, y, z),
                    grad3d(self.perm[p10], x - 1.0, y, z),
                ),
                lerp(
                    fx,
                    grad3d(self.perm[p01], x, y - 1.0, z),
                    grad3d(self.perm[p11], x - 1.0, y - 1.0, z),
                ),
            ),
            lerp(
                fy,
                lerp(
                    fx,
                    grad3d(self.perm[p00 + 1], x, y, z - 1.0),
                    grad3d(self.perm[p10 + 1], x - 1.0, y, z - 1.0),
                ),
                lerp(
                    fx,
                    grad3d(self.perm[p01 + 1], x, y - 1.0, z - 1.0),
                    grad3d(self.perm[p11 + 1], x - 1.0, y - 1.0, z - 1.0),
                ),
            ),
        )
    }

    fn noise2d(&self, mut x: f64, mut y: f64) -> f64 {
        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;

        x -= x.floor();
        y -= y.floor();

        let fx = (3.0 - 2.0 * x) * x * x;
        let fy = (3.0 - 2.0 * y) * y * y;
        let p0 = self.perm[x0] + y0;
        let p1 = self.perm[x0 + 1] + y0;

        lerp(
            fy,
            lerp(
                fx,
                grad2d(self.perm[p0], x, y),
                grad2d(self.perm[p1], x - 1.0, y),
            ),
            lerp(
                fx,
                grad2d(self.perm[p0 + 1], x, y - 1.0),
                grad2d(self.perm[p1 + 1], x - 1.0, y - 1.0),
            ),
        )
    }

    fn noise1d(&self, mut x: f64) -> f64 {
        let x0 = x.floor() as usize;

        x -= x.floor();

        x = fade(x);
        let fx = (3.0 - 2.0 * x) * x * x;
        lerp(
            fx,
            grad1d(self.perm[x0], x),
            grad1d(self.perm[x0 + 1], x + 1.0),
        )
    }
}

fn grad3d(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15;

    let u = if h < 8 { x } else { y };

    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };

    let u = if h & 1 == 0 { u } else { -u };

    let v = if h & 2 == 0 { v } else { -v };

    v + u
}

fn grad2d(hash: usize, x: f64, y: f64) -> f64 {
    let v = if hash & 1 == 0 { x } else { y };

    if (hash & 1) == 0 {
        -v
    } else {
        v
    }
}

fn grad1d(hash: usize, x: f64) -> f64 {
    if (hash & 1) == 0 {
        -x
    } else {
        x
    }
}

// Linear Interpolate
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}
// Fade function as defined by Ken Perlin.  This eases coordinate values
// so that they will "ease" towards integral values.  This ends up smoothing
// the final output.
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
