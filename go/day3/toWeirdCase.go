package kata

func toWeirdCase(str string) string {
	var result string
	wordIndex := 0

	for _, c := range str {
		if c == ' ' {
			result += " "
			wordIndex = 0
			continue
		}

		if wordIndex%2 == 0 && c >= 'a' {
			result += string(c - 32)
		} else if wordIndex%2 != 0 && c < 'a' {
			result += string(c + 32)
		} else {
			result += string(c)
		}
		wordIndex++
	}

	return result
}
