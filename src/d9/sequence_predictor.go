package d9

func getSumOfPredictedValues(sequences [][][]int, valueCalculation func([]int, int) int) int {
	total := 0

	for _, sequence := range sequences {
		sequenceTree := fillSequenceTree(sequence)
		nextValue := predictValue(sequenceTree, len(sequenceTree)-1, 0, calculateLastValueInSequence)
		total += nextValue
	}

	return total
}

func predictValue(sequenceValues [][]int, nextLevelIndex int, currentValue int, valueCalculation func([]int, int) int) int {
	nextLevel := sequenceValues[nextLevelIndex]
	nextValue := valueCalculation(nextLevel, currentValue)

	nextLevelIndex--

	if nextLevelIndex < 0 {
		return nextValue
	}

	return predictValue(sequenceValues, nextLevelIndex, nextValue, valueCalculation)
}
