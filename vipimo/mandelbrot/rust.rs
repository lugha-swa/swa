const UKUBWA: i32 = 600;
const KIWANGO_JUU: i32 = 200;

fn main() {
    let mut jumla_iter: i64 = 0;
    for py in 0..UKUBWA {
        for px in 0..UKUBWA {
            let x0 = (px as f64 * 3.5 / UKUBWA as f64) - 2.5;
            let y0 = (py as f64 * 2.0 / UKUBWA as f64) - 1.0;
            let mut x = 0.0f64;
            let mut y = 0.0f64;
            let mut iter = 0;
            let mut hai_bado = 1;
            while iter < KIWANGO_JUU && hai_bado == 1 {
                let x2 = x * x;
                let y2 = y * y;
                if (x2 + y2) > 4.0 {
                    hai_bado = 0;
                } else {
                    let xt = (x2 - y2) + x0;
                    y = (2.0 * x * y) + y0;
                    x = xt;
                    iter += 1;
                }
            }
            jumla_iter += iter as i64;
        }
    }
    println!("jumla_iter={}", jumla_iter);
}
