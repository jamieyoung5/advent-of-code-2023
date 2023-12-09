package d9

import (
	"fmt"
	"strconv"
	"strings"
)

func parseInput(inputs []string) [][][]int {
	var parsedInput [][][]int
	for _, input := range inputs {
		var sequenceValues [][]int
		inputFields := strings.Fields(input)

		var digits []int
		for _, field := range inputFields {
			num, err := strconv.Atoi(field)
			if err != nil {
				fmt.Println("Error converting string to integer:", err)
				continue
			}
			digits = append(digits, num)
		}

		sequenceValues = append(sequenceValues, digits)
		parsedInput = append(parsedInput, sequenceValues)
	}

	return parsedInput
}
