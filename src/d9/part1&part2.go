package d9

func Part1(inputs []string) int {
	sequences := parseInput(inputs)
	return getSumOfPredictedValues(sequences, calculateNextValueInSequence)
}

func Part2(inputs []string) int {
	sequences := parseInput(inputs)
	return getSumOfPredictedValues(sequences, calculateLastValueInSequence)
}

func calculateLastValueInSequence(nextLevel []int, currentValue int) int {
	return nextLevel[0] - currentValue
}

func calculateNextValueInSequence(nextLevel []int, currentValue int) int {
	return nextLevel[len(nextLevel)-1] + currentValue
}
