package leetcode

import "sort"

func relativeSortArray(arr1 []int, arr2 []int) []int {
	arrLen := len(arr1)
	mapped := make(map[int]int, arrLen)
	for _, num := range arr1 {
		mapped[num] += 1
	}
	res := make([]int, arrLen)
	idx := 0
	for _, num := range arr2 {
		for mapped[num] > 0 {
			res[idx] = num
			mapped[num] -= 1
			idx++
		}
	}
	sliceStart := idx
	for k, v := range mapped {
		for v > 0 {
			res[idx] = k
			v -= 1
			idx += 1
		}
	}
	sort.Ints(res[sliceStart:])
	return res
}
