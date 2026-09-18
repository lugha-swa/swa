package main

import "fmt"

const ukubwa = 600
const kiwangoJuu = 200

func main() {
	var jumlaIter int64 = 0
	for py := 0; py < ukubwa; py++ {
		for px := 0; px < ukubwa; px++ {
			x0 := (float64(px)*3.5/float64(ukubwa)) - 2.5
			y0 := (float64(py)*2.0/float64(ukubwa)) - 1.0
			x, y := 0.0, 0.0
			iter := 0
			haiBado := 1
			for iter < kiwangoJuu && haiBado == 1 {
				x2 := x * x
				y2 := y * y
				if (x2 + y2) > 4.0 {
					haiBado = 0
				} else {
					xt := (x2 - y2) + x0
					y = (2.0 * x * y) + y0
					x = xt
					iter++
				}
			}
			jumlaIter += int64(iter)
		}
	}
	fmt.Printf("jumla_iter=%d\n", jumlaIter)
}
