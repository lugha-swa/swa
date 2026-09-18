// miti_bst -- Rust reference. Iterative insert/search via raw pointers
// (mirrors the C/Swa algorithm exactly; avoids recursion-depth risk on
// an unbalanced tree, same reasoning as the other implementations).
use std::alloc::{alloc, Layout};

struct Nodi {
    thamani: i32,
    kushoto: *mut Nodi,
    kulia: *mut Nodi,
}

const N_INGIZA: i32 = 150000;
const N_TAFUTA: i32 = 150000;

static mut LCG_HALI: i64 = 98765;

fn nasibu_ijayo() -> i32 {
    unsafe {
        LCG_HALI = (LCG_HALI * 1103515245 + 12345) % 2147483648;
        LCG_HALI as i32
    }
}

unsafe fn nodi_mpya(v: i32) -> *mut Nodi {
    let layout = Layout::new::<Nodi>();
    let p = alloc(layout) as *mut Nodi;
    (*p).thamani = v;
    (*p).kushoto = std::ptr::null_mut();
    (*p).kulia = std::ptr::null_mut();
    p
}

unsafe fn ingiza(mzizi: *mut Nodi, v: i32) -> *mut Nodi {
    if mzizi.is_null() {
        return nodi_mpya(v);
    }
    let mut sasa = mzizi;
    loop {
        if v < (*sasa).thamani {
            if (*sasa).kushoto.is_null() {
                (*sasa).kushoto = nodi_mpya(v);
                return mzizi;
            }
            sasa = (*sasa).kushoto;
        } else {
            if (*sasa).kulia.is_null() {
                (*sasa).kulia = nodi_mpya(v);
                return mzizi;
            }
            sasa = (*sasa).kulia;
        }
    }
}

unsafe fn tafuta(mzizi: *mut Nodi, v: i32) -> i32 {
    let mut sasa = mzizi;
    let mut h: i32 = 0;
    while !sasa.is_null() {
        h += 1;
        if (*sasa).thamani == v {
            return h;
        }
        if v < (*sasa).thamani {
            sasa = (*sasa).kushoto;
        } else {
            sasa = (*sasa).kulia;
        }
    }
    0 - h
}

fn main() {
    unsafe {
        let mut mzizi: *mut Nodi = std::ptr::null_mut();
        for _ in 0..N_INGIZA {
            let v = nasibu_ijayo() % 20000000;
            mzizi = ingiza(mzizi, v);
        }

        let mut hatua_jumla: i32 = 0;
        let mut idadi_patikana: i32 = 0;
        for _ in 0..N_TAFUTA {
            let v = nasibu_ijayo() % 20000000;
            let r = tafuta(mzizi, v);
            if r > 0 {
                idadi_patikana += 1;
                hatua_jumla += r;
            } else {
                hatua_jumla -= r;
            }
        }

        println!("idadi_patikana={} hatua_jumla={}", idadi_patikana, hatua_jumla);
    }
}
