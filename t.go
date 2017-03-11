package main

import "fmt"

// fib returns a function that returns
// successive Fibonacci numbers.
func fibonacci(i int) int {
	if i < 2 {
		return i
	}
	return fibonacci(i - 2) + fibonacci(i - 1)
}

func main() {
	fmt.Println(fibonacci(45))
}

