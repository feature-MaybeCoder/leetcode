package leetcode

import (
	"strings"
)

func wordBreak(s string, wordDict []string) []string {
	res := make([]string, 0)
	cur := make([]string, 0)
	dfs(0, &s, &res, &cur, &wordDict)
	return res
}
func isWordsEqual(word1 *string, word2 *string) bool {
	if len(*word1) != len(*word2){
		return false
	}
	for i := 0; i < len(*word1); i++ {
		if (*word1)[i] != (*word2)[i] {
			return false
		}
	}
	return true
}
func dfs(idx int, s *string, res *[]string, stack *[]string, strs *[]string) {
	if idx >= len(*s) {
		*res = append(*res, strings.Join(*stack, " "))
		return
	}
	
	for i := idx; i < len(*s); i++ {
        slice := (*s)[idx:i+1]
		for j := 0; j < len(*strs); j++ {
			
			equal := isWordsEqual(&slice, &(*strs)[j])
			if !equal {
				continue
			}
			*stack = append(*stack, slice)
			dfs(i+1, s, res, stack, strs)
			*stack = (*stack)[:len(*stack)-1]
			break
		}
	}
}
