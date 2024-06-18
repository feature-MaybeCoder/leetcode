package leetcode

func minPatches(nums []int, n int) int {
	res := 0
	curSum := 0
	for i := 0; i < len(nums); i++ {
		prev := nums[i]
		for n > curSum && prev > curSum+1 {
			res += 1
			curSum += curSum + 1
		}
		curSum += prev
	}
	for curSum < n {
		res += 1
		curSum += curSum + 1
	}
	return res
}
