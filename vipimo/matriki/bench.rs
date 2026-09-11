const N: usize = 300;

fn main() {
    let mut a = vec![0i32; N * N];
    let mut b = vec![0i32; N * N];
    let mut c = vec![0i32; N * N];
    for i in 0..N * N {
        a[i] = (i % 97) as i32 + 1;
        b[i] = (i % 89) as i32 + 1;
    }
    for i in 0..N {
        for j in 0..N {
            let mut jumla: i32 = 0;
            for k in 0..N {
                jumla += a[i * N + k] * b[k * N + j];
            }
            c[i * N + j] = jumla;
        }
    }
    let mut hesabu: i64 = 0;
    for i in 0..N * N { hesabu += c[i] as i64; }
    println!("hesabu={} c00={}", hesabu, c[0]);
}
