package leetcode

import (
	"fmt"
	"math"
)

func minSwaps(nums []int) int {
	n := len(nums)
	left := 0
	right := 0
	zeroCount := 0
	minRes := math.MaxInt
	oneCount := 0

	for _, n := range nums {
		if n == 1 {
			oneCount++
		}
	}
	for right < oneCount {
		if nums[right] == 0 {
			zeroCount++
		}
		right++
	}
	right--
	for right < n {
		minRes = min(minRes, zeroCount)
		right++
		if right >= n {
			break
		}
		if nums[left] == 0 {
			zeroCount--
		}
		left++
		if nums[right] == 0 {
			zeroCount++
		}
	}
	right = left
	left = 0

	for right < n {
		minRes = min(minRes, zeroCount)
		if nums[right] == 0 {
			zeroCount--
		}
		if nums[left] == 0 {
			zeroCount++
		}
		right++
		left++
	}

	return minRes
}

func min(n1, n2 int) int {
	if n1 < n2 {
		return n1
	}
	return n2
}
