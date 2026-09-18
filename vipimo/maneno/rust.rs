const IDADI_MANENO: i32 = 400000;
const IDADI_KAMUSI: i32 = 16;

static mut LCG_HALI: i64 = 555777;

fn nasibu_ijayo() -> i32 {
    unsafe {
        LCG_HALI = (LCG_HALI * 1103515245 + 12345) % 2147483648;
        LCG_HALI as i32
    }
}

fn kamusi_neno(idx: i32) -> &'static str {
    match idx {
        0 => "paka",
        1 => "mbwa",
        2 => "ndege",
        3 => "samaki",
        4 => "nyoka",
        5 => "tembo",
        6 => "simba",
        7 => "chui",
        8 => "nyani",
        9 => "kobe",
        10 => "sungura",
        11 => "kuku",
        12 => "bata",
        13 => "mbuzi",
        14 => "ngamia",
        _ => "farasi",
    }
}

fn main() {
    let mut maandishi: Vec<u8> = Vec::new();
    for _ in 0..IDADI_MANENO {
        let idx = nasibu_ijayo() % IDADI_KAMUSI;
        let neno = kamusi_neno(idx);
        maandishi.extend_from_slice(neno.as_bytes());
        maandishi.push(b' ');
    }

    let mut hesabu = [0i32; 16];
    let wapi = maandishi.len();
    let mut p = 0usize;
    while p < wapi {
        let mwanzo = p;
        while p < wapi && maandishi[p] != b' ' {
            p += 1;
        }
        let urefu_neno = (p - mwanzo) as i32;
        for idx in 0..IDADI_KAMUSI {
            let kn = kamusi_neno(idx);
            if kn.len() as i32 == urefu_neno && kn.as_bytes() == &maandishi[mwanzo..p] {
                hesabu[idx as usize] += 1;
                break;
            }
        }
        p += 1;
    }

    let mut cheki: i64 = 0;
    for idx in 0..16 {
        cheki += hesabu[idx] as i64 * (idx as i64 + 1);
    }

    println!("cheki={} hesabu0={} hesabu15={}", cheki, hesabu[0], hesabu[15]);
}
