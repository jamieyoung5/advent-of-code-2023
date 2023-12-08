package d8

type Nodes map[string][2]string

func Part1(inputs []string) int {
	nodes, directions := parseInput(inputs)
	return calculateDistance("AAA", "ZZZ", nodes, directions)
}
