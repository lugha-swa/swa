package main

import "fmt"

const nKipimo = 25000000

func main() {
	n := nKipimo
	mchujo := make([]byte, n+1)
	mchujo[0] = 1
	mchujo[1] = 1
	for i := 2; i*i <= n; i++ {
		if mchujo[i] == 0 {
			for j := i * i; j <= n; j += i {
				mchujo[j] = 1
			}
		}
	}
	idadiKuu := 0
	for i := 0; i <= n; i++ {
		if mchujo[i] == 0 {
			idadiKuu++
		}
	}
	fmt.Printf("idadi_kuu=%d\n", idadiKuu)
}
