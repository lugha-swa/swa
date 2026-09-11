struct Mfumo { thamani: i32, kiwango: i32 }

impl Mfumo {
    fn new(mwanzo: i32) -> Box<Mfumo> { Box::new(Mfumo { thamani: mwanzo, kiwango: 1 }) }
    fn pata_thamani(&self) -> i32 { self.thamani }
    fn pata_kiwango(&self) -> i32 { self.kiwango }
    fn sasisha(&mut self, pembejeo: i32) {
        let sasa = self.pata_thamani();
        let kw = self.pata_kiwango();
        let mut mpya = sasa + (pembejeo % 7) - (kw % 3);
        if mpya < 0 { mpya = -mpya; }
        self.thamani = mpya;
        if mpya % 500 == 0 { self.kiwango = kw + 1; }
    }
}

struct Mchezo { m: Vec<Box<Mfumo>> }

impl Mchezo {
    fn new() -> Mchezo {
        let starts = [10,20,30,40,50,60,70,80,90,100,110,120];
        Mchezo { m: starts.iter().map(|&s| Mfumo::new(s)).collect() }
    }
    fn sasisha(&mut self, zamu: i32) {
        for i in 0..12 {
            self.m[i].sasisha(zamu + i as i32 + 1);
        }
    }
}

const ZAMU_IDADI: i32 = 200000;

fn main() {
    let mut g = Mchezo::new();
    let mut z = 0;
    while z < ZAMU_IDADI {
        g.sasisha(z);
        z += 1;
    }
    let jumla = g.m[0].pata_thamani() + g.m[5].pata_thamani() + g.m[11].pata_thamani();
    println!("jumla={} z={}", jumla, z);
}
