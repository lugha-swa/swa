package main

import "fmt"

const n = 300

func main() {
	a := make([]int32, n*n)
	b := make([]int32, n*n)
	c := make([]int32, n*n)
	for i := 0; i < n*n; i++ {
		a[i] = int32(i%97) + 1
		b[i] = int32(i%89) + 1
	}
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			var jumla int32 = 0
			for k := 0; k < n; k++ {
				jumla += a[i*n+k] * b[k*n+j]
			}
			c[i*n+j] = jumla
		}
	}
	var hesabu int64 = 0
	for i := 0; i < n*n; i++ {
		hesabu += int64(c[i])
	}
	fmt.Printf("hesabu=%d c00=%d\n", hesabu, c[0])
}
