package main

import "fmt"

const kiasi = 1000000

var randHali uint64 = 4101842887655102017

func nasibuIjayo() int32 {
	randHali ^= randHali >> 32
	randHali *= 2685821657736338717
	randHali ^= randHali >> 32
	return int32(randHali % 1000000000)
}

func badilisha(arr []int32, i, j int) {
	arr[i], arr[j] = arr[j], arr[i]
}

func gawanya(arr []int32, chini, juu int) int {
	kati := chini + (juu-chini)/2
	if arr[kati] < arr[chini] {
		badilisha(arr, kati, chini)
	}
	if arr[juu] < arr[chini] {
		badilisha(arr, juu, chini)
	}
	if arr[juu] < arr[kati] {
		badilisha(arr, juu, kati)
	}
	badilisha(arr, kati, juu)
	pivot := arr[juu]
	i := chini - 1
	j := chini
	for j < juu {
		if arr[j] < pivot {
			i++
			badilisha(arr, i, j)
		}
		j++
	}
	badilisha(arr, i+1, juu)
	return i + 1
}

func panga(arr []int32, chini, juu int) {
	if chini < juu {
		p := gawanya(arr, chini, juu)
		panga(arr, chini, p-1)
		panga(arr, p+1, juu)
	}
}

func main() {
	arr := make([]int32, kiasi)
	for i := 0; i < kiasi; i++ {
		arr[i] = nasibuIjayo()
	}
	panga(arr, 0, kiasi-1)
	sahihi := 1
	for i := 0; i < kiasi-1; i++ {
		if arr[i] > arr[i+1] {
			sahihi = 0
		}
	}
	fmt.Printf("sahihi=%d kwanza=%d mwisho=%d\n", sahihi, arr[0], arr[kiasi-1])
}
