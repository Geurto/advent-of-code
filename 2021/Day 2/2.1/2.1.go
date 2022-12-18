package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	x, y := 0, 0
	input := bufio.NewScanner(os.Stdin)
	for input.Scan() {
		line := input.Text()
		arg, _ := strconv.Atoi(strings.Fields(line)[1])
		switch line[0] {
		case 'f':
			x += arg
		case 'u':
			y += arg
		case 'd':
			y -= arg
		}
	}
	fmt.Println(x * y)
}
