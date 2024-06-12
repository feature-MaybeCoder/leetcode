package leetcode

import "math"

func increasingTriplet(nums []int) bool {
	small := math.MaxInt
	big := math.MaxInt
	for _, num := range nums {
		if num <= small {
			small = num
			continue
		}
		if num <= big {
			big = num
			continue
		}
		return true
	}
	return false
}
