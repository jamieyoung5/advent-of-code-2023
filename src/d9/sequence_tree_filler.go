package d9

func fillSequenceTree(sequenceValues [][]int) [][]int {
	nextSequence := generateNextSequence(sequenceValues[len(sequenceValues)-1])
	sequenceValues = append(sequenceValues, nextSequence)
	if checkForZeros(nextSequence) {
		return sequenceValues
	}

	return fillSequenceTree(sequenceValues)
}

func checkForZeros(slice []int) bool {
	for _, element := range slice {
		if element != 0 {
			return false
		}
	}

	return true
}

func generateNextSequence(sequence []int) []int {
	var nextSequence []int
	for i := 0; i < len(sequence)-1; i++ {
		product := sequence[i+1] - sequence[i]

		nextSequence = append(nextSequence, product)
	}

	return nextSequence
}
