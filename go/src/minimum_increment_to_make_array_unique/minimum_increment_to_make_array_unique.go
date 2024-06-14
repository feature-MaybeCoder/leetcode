package leetcode

import (
	"sort"
)

func minIncrementForUnique(nums []int) int {
	res := 0
	sort.Ints(nums)

	for i := 1; i < len(nums); i++ {
		prev := nums[i-1]
		cur := nums[i]
		// if there is no duplicates with prev
		if cur > prev {
			continue
		}
		res += prev - cur + 1
		nums[i] = prev +1
	}
	
	return res
}
