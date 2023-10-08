use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH};

fn donut(x: f64, y: f64, z: f64) -> f64 {
    let radius = 0.4;
    let thickness = 0.3;
    (((x.powi(2) + y.powi(2)).sqrt() - radius).powi(2) + z.powi(2)).sqrt() - thickness / 2.0
}

const SDF: fn(f64, f64, f64) -> f64 = donut;

fn normal(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let e = 0.001;
    let n_x = SDF(x + e, y, z) - SDF(x - e, y, z);
    let n_y = SDF(x, y + e, z) - SDF(x, y - e, z);
    let n_z = SDF(x, y, z + e) - SDF(x, y, z - e);
    let norm = (n_x.powi(2) + n_y.powi(2) + n_z.powi(2)).sqrt();
    (n_x / norm, n_y / norm, n_z / norm)
}

fn sample(x: f64, y: f64) -> char {
    let mut z = -10.0;
    for _ in 0..30 {
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let o: f64 = (since_epoch.as_secs() as f64 + (since_epoch.subsec_nanos() as f64 * 1e-9)) * 2.0;
        let t_x = x * o.cos() - z * o.sin();
        let t_z = x * o.sin() + z * o.cos();
        let d = SDF(t_x, y, t_z);
        if d <= 0.01 {
            let (_, nt_y, nt_z) = normal(t_x, y, t_z);
            let is_lit = nt_y < -0.15;
            let is_frosted = nt_z < -0.5;

            if is_frosted {
                return if is_lit { '@' } else { '#' };
            } else {
                return if is_lit { '=' } else { '.' };
            }
        } else {
            z += d;
        }
    }
    ' '
}

fn main() {
    loop {
        let mut line = String::new();
        for y in 0..20 {
            for x in 0..80 {
                let remapped_x: f64 = x as f64 / 80.0 * 2.0 - 1.0;
                let remapped_y: f64 = (y as f64 / 20.0 * 2.0 - 1.0) * (2.0 * 20.0 / 80.0);
                line += &sample(remapped_x, remapped_y).to_string();
            }
            line += "\n";
        }
        print!("\x1b[2J{line}");
        sleep(std::time::Duration::from_secs_f64(1.0 / 75.0));
    }
}
