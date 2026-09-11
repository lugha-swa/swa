const IDADI: i64 = 2_000_000;

fn heshi_fnv(s: &[u8], urefu: i32) -> i32 {
    let mut h: i32 = -2128831035;
    for i in 0..urefu {
        h ^= s[i as usize] as i32;
        h = h.wrapping_mul(16777619);
    }
    h
}

fn main() {
    let neno = b"kompyuta_ya_mfano_kwa_uzito_wa_kati_12345";
    let urefu = neno.len() as i32;
    let mut jumla: i64 = 0;
    for i in 0..IDADI {
        let h = heshi_fnv(neno, urefu);
        jumla += h as i64 + i;
    }
    println!("jumla={} urefu={}", jumla, urefu);
}
