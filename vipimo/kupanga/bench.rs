const KIASI: usize = 1_000_000;

struct Rng { state: u64 }
impl Rng {
    fn next(&mut self) -> i32 {
        self.state ^= self.state >> 32;
        self.state = self.state.wrapping_mul(2685821657736338717u64);
        self.state ^= self.state >> 32;
        (self.state % 1_000_000_000) as i32
    }
}

fn badilisha(arr: &mut [i32], i: usize, j: usize) {
    arr.swap(i, j);
}

fn gawanya(arr: &mut [i32], chini: usize, juu: usize) -> usize {
    let kati = chini + (juu - chini) / 2;
    if arr[kati] < arr[chini] { badilisha(arr, kati, chini); }
    if arr[juu] < arr[chini] { badilisha(arr, juu, chini); }
    if arr[juu] < arr[kati] { badilisha(arr, juu, kati); }
    badilisha(arr, kati, juu);
    let pivot = arr[juu];
    let mut i: i64 = chini as i64 - 1;
    let mut j = chini;
    while j < juu {
        if arr[j] < pivot { i += 1; badilisha(arr, i as usize, j); }
        j += 1;
    }
    badilisha(arr, (i + 1) as usize, juu);
    (i + 1) as usize
}

fn panga(arr: &mut [i32], chini: i64, juu: i64) {
    if chini < juu {
        let p = gawanya(arr, chini as usize, juu as usize) as i64;
        panga(arr, chini, p - 1);
        panga(arr, p + 1, juu);
    }
}

fn main() {
    let mut rng = Rng { state: 4101842887655102017u64 };
    let mut arr: Vec<i32> = (0..KIASI).map(|_| rng.next()).collect();
    panga(&mut arr, 0, KIASI as i64 - 1);
    let mut sahihi = 1;
    for i in 0..KIASI - 1 { if arr[i] > arr[i+1] { sahihi = 0; } }
    println!("sahihi={} kwanza={} mwisho={}", sahihi, arr[0], arr[KIASI-1]);
}
