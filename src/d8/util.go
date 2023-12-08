package d8

import "strings"

/*Calculates the amount of steps needed to get from 'firstNode' to any node ending with 'suffix'*/
func calculateDistance(firstNode string, suffix string, nodes Nodes, directions []rune) int {
	currentNode := firstNode
	steps := 0
	directionIndex := 0

	for !allEndWith([]string{currentNode}, suffix) {
		steps++

		direction := directions[directionIndex]
		currentNode = nodes[currentNode][convertDirection(direction)]
		directionIndex = (directionIndex + 1) % len(directions)

	}

	return steps
}

/*given a slice and a suffix, returns true or false depending on if every element in the given slice ends in the given suffix*/
func allEndWith(slice []string, suffix string) bool {
	for _, str := range slice {
		if !strings.HasSuffix(str, suffix) {
			return false
		}
	}
	return true
}

func convertDirection(direction int32) int {
	switch direction {
	case 'R':
		return 1
	case 'L':
		return 0
	default:
		return -1
	}
}
