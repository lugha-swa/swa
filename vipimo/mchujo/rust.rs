const N_KIPIMO: i32 = 25000000;

fn main() {
    let n = N_KIPIMO;
    let mut mchujo = vec![0u8; (n + 1) as usize];
    mchujo[0] = 1;
    mchujo[1] = 1;
    let mut i: i32 = 2;
    while i * i <= n {
        if mchujo[i as usize] == 0 {
            let mut j = i * i;
            while j <= n {
                mchujo[j as usize] = 1;
                j += i;
            }
        }
        i += 1;
    }
    let mut idadi_kuu: i32 = 0;
    for i in 0..=n {
        if mchujo[i as usize] == 0 {
            idadi_kuu += 1;
        }
    }
    println!("idadi_kuu={}", idadi_kuu);
}
