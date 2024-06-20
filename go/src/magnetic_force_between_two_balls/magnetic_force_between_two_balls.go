package leetcode

import (
	"math"
	"sort"
)

func maxDistance(position []int, m int) int {
	left := 1
	sort.Ints(position)
	right := position[len(position)-1]

	for left < right {
		mid := (left + right + 1) >> 1
		if bestFit(position, mid, m) {
			left = mid
			continue
		}
		right = mid - 1
	}
	return left
}

func bestFit(position []int, target, m int) bool {
	prev := math.MinInt32
	fited := 0

	for _, num := range position {
		diff := num - prev
		if diff >= target {
			prev = num
			fited++
		}
	}
	return fited >= m
}

func abs(num int) int {
	if num < 0 {
		return -num
	}
	return num
}
func Min(num, num1 int) int {
	if num < num1 {
		return num
	}
	return num1
}
