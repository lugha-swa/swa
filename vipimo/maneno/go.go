package main

import (
	"bytes"
	"fmt"
)

const idadiManeno = 400000
const idadiKamusi = 16

var lcgHali int64 = 555777

func nasibuIjayo() int32 {
	lcgHali = (lcgHali*1103515245 + 12345) % 2147483648
	return int32(lcgHali)
}

func kamusiNeno(idx int32) string {
	switch idx {
	case 0:
		return "paka"
	case 1:
		return "mbwa"
	case 2:
		return "ndege"
	case 3:
		return "samaki"
	case 4:
		return "nyoka"
	case 5:
		return "tembo"
	case 6:
		return "simba"
	case 7:
		return "chui"
	case 8:
		return "nyani"
	case 9:
		return "kobe"
	case 10:
		return "sungura"
	case 11:
		return "kuku"
	case 12:
		return "bata"
	case 13:
		return "mbuzi"
	case 14:
		return "ngamia"
	default:
		return "farasi"
	}
}

func main() {
	var maandishi []byte
	for i := 0; i < idadiManeno; i++ {
		idx := nasibuIjayo() % idadiKamusi
		neno := kamusiNeno(idx)
		maandishi = append(maandishi, []byte(neno)...)
		maandishi = append(maandishi, ' ')
	}

	var hesabu [16]int32
	wapi := len(maandishi)
	p := 0
	for p < wapi {
		mwanzo := p
		for p < wapi && maandishi[p] != ' ' {
			p++
		}
		urefuNeno := int32(p - mwanzo)
		for idx := int32(0); idx < idadiKamusi; idx++ {
			kn := kamusiNeno(idx)
			if int32(len(kn)) == urefuNeno && bytes.Equal([]byte(kn), maandishi[mwanzo:p]) {
				hesabu[idx]++
				break
			}
		}
		p++
	}

	var cheki int64 = 0
	for idx := 0; idx < 16; idx++ {
		cheki += int64(hesabu[idx]) * int64(idx+1)
	}

	fmt.Printf("cheki=%d hesabu0=%d hesabu15=%d\n", cheki, hesabu[0], hesabu[15])
}
