package leetcode

import (
	"fmt"
	"math"
)

func strangePrinter(s string) int {
	n := len(s)
	if n == 0 {
		return 0
	}
	dp := make([][]int, n)
	for i := 0; i < n; i++ {
		dp[i] = make([]int, n)
		dp[i][i] = 1
	}
	for i := n - 1; i >= 0; i-- {
		for dist := 1; dist+i < n; dist++ {
			j := dist + i
			if dist == 1 {
				if s[i] == s[j] {
					dp[i][j] = 1
				} else {
					dp[i][j] = 2
				}
				continue
			}
			dp[i][j] = math.MaxInt
			for k := i; k+1 <= j; k++ {
				tmp := dp[i][k] + dp[k+1][j]
				dp[i][j] = min(dp[i][j], tmp)
			}
			if s[i] == s[j] {
				dp[i][j]--
			}
		}
	}
	for _, c := range dp {
		fmt.Println(c)
	}
	return dp[0][n-1]
}
func min(n1, n2 int) int {
	if n1 < n2 {
		return n1
	}
	return n2
}
