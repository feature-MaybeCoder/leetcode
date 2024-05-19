package leetcode

import "math"
func min(num1 int64, num2 int64)int64{
	if num1 > num2{
		return num2
	}
	return num1
}
func abs(num1 int64)int64{
	if num1 < 0{
		return num1 * -1
	}
	return num1
}
func maximumValueSum(nums []int, k int, edges [][]int) int64 {
	var max int64 = 0
	count := 0
	var sac int64 = math.MaxInt64
	k64 := int64(k)
	for i := 0; i < len(nums); i++ {
		num := int64(nums[i])
		xor := num ^ k64
		sac = min(sac, abs(num - xor))
		if xor > num{
			max += xor
			count += 1
			continue
		}
		max += num
	}	
	if count % 2 ==0{
		return max
	}
	return max - sac
}

