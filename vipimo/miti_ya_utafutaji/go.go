package main

import "fmt"

const nIngiza = 150000
const nTafuta = 150000

type Nodi struct {
	thamani int32
	kushoto *Nodi
	kulia   *Nodi
}

var lcgHali int64 = 98765

func nasibuIjayo() int32 {
	lcgHali = (lcgHali*1103515245 + 12345) % 2147483648
	return int32(lcgHali)
}

func nodiMpya(v int32) *Nodi {
	return &Nodi{thamani: v}
}

func ingiza(mzizi *Nodi, v int32) *Nodi {
	if mzizi == nil {
		return nodiMpya(v)
	}
	sasa := mzizi
	for {
		if v < sasa.thamani {
			if sasa.kushoto == nil {
				sasa.kushoto = nodiMpya(v)
				return mzizi
			}
			sasa = sasa.kushoto
		} else {
			if sasa.kulia == nil {
				sasa.kulia = nodiMpya(v)
				return mzizi
			}
			sasa = sasa.kulia
		}
	}
}

func tafuta(mzizi *Nodi, v int32) int32 {
	sasa := mzizi
	var h int32 = 0
	for sasa != nil {
		h++
		if sasa.thamani == v {
			return h
		}
		if v < sasa.thamani {
			sasa = sasa.kushoto
		} else {
			sasa = sasa.kulia
		}
	}
	return 0 - h
}

func main() {
	var mzizi *Nodi = nil
	for i := 0; i < nIngiza; i++ {
		v := nasibuIjayo() % 20000000
		mzizi = ingiza(mzizi, v)
	}

	var hatuaJumla int32 = 0
	var idadiPatikana int32 = 0
	for i := 0; i < nTafuta; i++ {
		v := nasibuIjayo() % 20000000
		r := tafuta(mzizi, v)
		if r > 0 {
			idadiPatikana++
			hatuaJumla += r
		} else {
			hatuaJumla -= r
		}
	}

	fmt.Printf("idadi_patikana=%d hatua_jumla=%d\n", idadiPatikana, hatuaJumla)
}
