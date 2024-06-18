package leetcode

import (
	"sort"
)

type Job struct {
	difficulty int
	profit     int
}

func Max(num1, num2 int) int {
	if num1 > num2 {
		return num1
	}
	return num2
}

func maxProfitAssignment(difficulty []int, profit []int, worker []int) int {
	dfLen := len(difficulty)
	adj := make([]Job, dfLen)
	for idx, df := range difficulty {
		adj[idx] = Job{
			difficulty: df,
			profit:     profit[idx],
		}
	}
	sort.Slice(adj, func(i, j int) bool {
		return adj[i].difficulty < adj[j].difficulty
	})
	sort.Ints(worker)
	jobIdx := 0
	maxProfit := 0
	res := 0
	for _, work := range worker {
		for jobIdx < dfLen && adj[jobIdx].difficulty <= work {
			maxProfit = Max(adj[jobIdx].profit, maxProfit)
			jobIdx++
		}
		res += maxProfit
	}
	return res
}
