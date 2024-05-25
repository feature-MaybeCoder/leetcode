package leetcode

func maxScoreWords(words []string, letters []byte, score []int) int {
	var countedLetters [26]int = [26]int{}
	var mappedWords [][26]int = make([][26]int, len(words))
	lowerShift := 97
	for i := 0; i < len(letters); i++ {
		countedLetters[int(letters[i]) - lowerShift] += 1
	}
	for i := 0; i < len(words); i++ {
		for j := 0; j < len(words[i]); j++ {
			mappedWords[i][int(words[i][j]) - lowerShift] +=1
		}
	}
	return dfs(0, &mappedWords, &countedLetters, &score)
}
func canAddWord(word *[26]int, countedLetters *[26]int)bool{
	for i := 0; i < 26; i++ {
		if (*word)[i] > (*countedLetters)[i]{
			return false
		}
	}
	return true
}
func addWord(word *[26]int, countedLetters *[26]int, score *[]int)int{
	sum := 0
	for i := 0; i < 26; i++ {
		(*countedLetters)[i] -= (*word)[i]
		sum += (*score)[i] * (*word)[i]
	}
	return sum
}
func removeWord(word *[26]int, countedLetters *[26]int){
	for i := 0; i < 26; i++ {
		(*countedLetters)[i] += (*word)[i]
	}
}
func max(num1 int, num2 int)int{
	if num1 > num2{
		return num1
	}
	return num2
}
func dfs(idx int, words *[][26]int, countedLetters *[26]int, score *[]int)int{
	if idx >= len(*words){
		return 0
	}
	canAdd := canAddWord(&(*words)[idx], countedLetters)
	res := dfs(idx +1, words, countedLetters, score)
	if !canAdd{
		return res
	}
	curRes := addWord(&(*words)[idx], countedLetters, score)
	executed := dfs(idx +1, words, countedLetters, score)
	removeWord(&(*words)[idx], countedLetters)
	return max(res, executed + curRes)
}