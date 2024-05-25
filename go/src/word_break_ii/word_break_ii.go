package leetcode

import (
	"strings"
)

func wordBreak(s string, wordDict []string) []string {
	mapped := make(map[string]bool, 0)
	res := make([]string, 0)
	cur := make([]string, 0)
	for _, word := range wordDict {
		mapped[word] = true
	}
	dfs(0, &s, &res, &cur, &mapped)
	return res
}
func dfs(idx int, s *string, res *[]string, stack *[]string, mapped *map[string]bool) {
	if idx >= len(*s) {
		*res = append(*res, strings.Join(*stack, " "))
		return
	}

	for i := idx; i < len(*s); i++ {
		slice := (*s)[idx : i+1]
		_, contains := (*mapped)[slice]
		if contains {
			*stack = append(*stack, slice)
			dfs(i+1, s, res, stack, mapped)
			*stack = (*stack)[:len(*stack)-1]
		}
	}
}
