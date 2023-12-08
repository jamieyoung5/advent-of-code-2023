package d8

import (
	"fmt"
	"strings"
)

func Part2(inputs []string) int {
	nodes, directions := parseInput(inputs)
	startNodes := getAllStartNodes(nodes)
	steps := 0

	resultsCh := make(chan int, len(startNodes))
	for _, key := range startNodes {
		go func(k string) {
			result := calculateDistance(k, "Z", nodes, directions)
			resultsCh <- result
		}(key)
	}

	var allSteps []int
	for range startNodes {
		allSteps = append(allSteps, <-resultsCh)
	}
	close(resultsCh)

	fmt.Println(allSteps)

	steps = allSteps[0]
	for _, step := range allSteps[1:] {
		steps = lcm(steps, step)
	}
	return steps
}

/*Given two numbers, a & b, calculates the lowest common multiple between a and b*/
func lcm(a, b int) int {
	return a / gcd(a, b) * b
}

/*Given two numbers, a & b, calculates the greatest common divisor between a and b*/
func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

/* Given all the nodes, will return a list of node keys that start with A*/
func getAllStartNodes(nodes Nodes) []string {
	var startNodes []string
	for key, _ := range nodes {
		if strings.HasSuffix(key, "A") {
			startNodes = append(startNodes, key)
		}
	}

	return startNodes
}
