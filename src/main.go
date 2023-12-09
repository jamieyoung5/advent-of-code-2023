package main

import (
	"advent-of-code-2023/src/d8"
	"advent-of-code-2023/src/d9"
	"bufio"
	"fmt"
	"os"
)

func main() {
	challengeResult := "Day %d Part 1: %d, Part 2: %d\n"

	day8Input, _ := readFileLines("inputs/d8.txt")
	day8Part1 := d8.Part1(day8Input)
	day8Part2 := d8.Part2(day8Input)
	fmt.Printf(challengeResult, 8, day8Part1, day8Part2)

	day9Input, _ := readFileLines("inputs/d9.txt")
	day9Part1 := d9.Part1(day9Input)
	day9Part2 := d9.Part2(day9Input)
	fmt.Printf(challengeResult, 9, day9Part1, day9Part2)
}

func readFileLines(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}
