package leetcode

import (
	"fmt"
	"math"
)

func stoneGameII(piles []int) int {
	n := len(piles)
	cache := make([]map[int]int, n)
	postfix := make([]int, n)
	postfix[n-1] = piles[n-1]
	for i := n - 2; i >= 0; i-- {
		postfix[i] = postfix[i+1] + piles[i]
	}
	for i := 0; i < n; i++ {
		cache[i] = map[int]int{}
	}
	var dp func(i, m int) int
	dp = func(i, m int) int {
		if i == n {
			return 0
		}
		if i+2*m >= n {
			return postfix[i]
		}
		if _, ok := cache[i][m]; ok {
			return cache[i][m]
		}
		minRes := math.MaxInt
		for j := 1; j <= 2*m; j++ {
			minRes = min(dp(i+j, max(j, m)), minRes)
		}
		res := postfix[i] - minRes
		cache[i][m] = res
		return res
	}
	
	return dp(0, 1)
}

func max(n1, n2 int) int {
	if n1 > n2 {
		return n1
	}
	return n2
}
func min(n1, n2 int) int {
	if n1 < n2 {
		return n1
	}
	return n2
}
