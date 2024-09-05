package leetcode

import "runtime/debug"

func missingRolls(rolls []int, mean int, n int) []int {
	rollsLen := len(rolls)
	rollsSum := 0
	for _, num := range rolls {
		rollsSum += num
	}
	targetSum := (mean * (n + rollsLen)) - rollsSum
	if targetSum < n || targetSum > n*6 {
		return nil
	}
	averageTargetSum := targetSum / n
	res := make([]int, n)
	for i := 0; i < n; i++ {
		selectedDice := averageTargetSum
		res[i] = selectedDice
		targetSum -= selectedDice
	}
	for i := 0; i < n && targetSum > 0; i++ {
		maxDiff := 6 - res[i]
		for maxDiff > targetSum {
			maxDiff--
		}
		res[i] += maxDiff
		targetSum -= maxDiff
	}

	return res
}

func init() {
	debug.SetGCPercent(-1)
}
