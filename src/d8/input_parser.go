package d8

import "strings"

func parseInput(inputs []string) (Nodes, []rune) {
	directions := inputs[0]
	inputs = inputs[2:]

	nodes := make(Nodes)

	for _, input := range inputs {
		splitInput := strings.Split(input, " = ")
		key := splitInput[0]

		valuesRaw := strings.ReplaceAll(splitInput[1], "(", "")
		valuesRaw = strings.ReplaceAll(valuesRaw, ")", "")
		values := strings.Split(valuesRaw, ", ")

		nodes[key] = [2]string{values[0], values[1]}
	}

	return nodes, []rune(directions)
}
