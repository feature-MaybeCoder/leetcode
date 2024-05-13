package leetcode

import "sort"
func maximumHappinessSum(happiness []int, k int) int64 {
    sort.Slice(happiness, func(i, j int) bool {
		return happiness[i] > happiness[j]
	})
	picked := 0
	var sum int64 = 0
	for i := 0; i < k; i++ {
		if (picked >= happiness[i]){
			break
		}
		sum += int64(happiness[i] - picked)
		picked += 1
	}

	return sum
}

