package leetcode

func longestSubarray(nums []int, limit int) int {
	min := 0
	max := 0
	left := 0
	right := 0
	res := 0
	for right < len(nums) {
		if left == right {
			right++
			continue
		}
		if nums[right] <= nums[min] {
			min = right
		}
		if nums[right] >= nums[max] {
			max = right
		}

		diff := abs(nums[max] - nums[min])
		if diff <= limit {
			right++
			continue
		}
		res = Max(res, right-left)
		left = Min(min, max) + 1
		min = left
		max = left
		right = left + 1
		left++
	}

	res = Max(res, right-left)
	return res
}

func Min(num, num1 int) int {
	if num < num1 {
		return num
	}
	return num1
}

func Max(num, num1 int) int {
	if num > num1 {
		return num
	}
	return num1
}

func abs(num int) int {
	if num < 0 {
		return -num
	}
	return num
}
