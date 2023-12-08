package main

import (
	"advent-of-code-2023/src/d8"
	"bufio"
	"fmt"
	"os"
)

func main() {

	day8Input, err := readFileLines("inputs/d8.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	day8Part1 := d8.Part1(day8Input)
	day8Part2 := d8.Part2(day8Input)
	fmt.Printf("Day 8 Part 1: %d, Part 2: %d", day8Part1, day8Part2)
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
