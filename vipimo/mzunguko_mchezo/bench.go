package main

import "fmt"

type Mfumo struct {
	thamani int32
	kiwango int32
}

func mfumoUnda(mwanzo int32) *Mfumo { return &Mfumo{thamani: mwanzo, kiwango: 1} }
func (m *Mfumo) pataThamani() int32 { return m.thamani }
func (m *Mfumo) pataKiwango() int32 { return m.kiwango }
func (m *Mfumo) sasisha(pembejeo int32) {
	sasa := m.pataThamani()
	kw := m.pataKiwango()
	mpya := sasa + (pembejeo % 7) - (kw % 3)
	if mpya < 0 {
		mpya = -mpya
	}
	m.thamani = mpya
	if mpya%500 == 0 {
		m.kiwango = kw + 1
	}
}

type Mchezo struct {
	m [12]*Mfumo
}

func mchezoUnda() *Mchezo {
	starts := [12]int32{10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120}
	g := &Mchezo{}
	for i, s := range starts {
		g.m[i] = mfumoUnda(s)
	}
	return g
}

func (g *Mchezo) sasisha(zamu int32) {
	for i := 0; i < 12; i++ {
		g.m[i].sasisha(zamu + int32(i) + 1)
	}
}

const zamuIdadi = 200000

func main() {
	g := mchezoUnda()
	var z int32 = 0
	for z < zamuIdadi {
		g.sasisha(z)
		z++
	}
	jumla := g.m[0].pataThamani() + g.m[5].pataThamani() + g.m[11].pataThamani()
	fmt.Printf("jumla=%d z=%d\n", jumla, z)
}
