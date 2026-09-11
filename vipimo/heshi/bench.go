package main

import "fmt"

const idadi = 2000000

func heshiFnv(s []byte, urefu int32) int32 {
	h := int32(-2128831035)
	for i := int32(0); i < urefu; i++ {
		h ^= int32(s[i])
		h *= 16777619
	}
	return h
}

func main() {
	neno := []byte("kompyuta_ya_mfano_kwa_uzito_wa_kati_12345")
	urefu := int32(len(neno))
	var jumla int64 = 0
	for i := int64(0); i < idadi; i++ {
		h := heshiFnv(neno, urefu)
		jumla += int64(h) + i
	}
	fmt.Printf("jumla=%d urefu=%d\n", jumla, urefu)
}
