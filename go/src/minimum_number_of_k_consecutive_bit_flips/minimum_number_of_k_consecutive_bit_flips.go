package leetcode

func minKBitFlips(nums []int, k int) int {
	q := make([]int, 0, len(nums))
	right := 0
	res := 0
	for right < len(nums) {
		for len(q) > 0 && right > k+q[0]-1 {
			q = q[1:]
		}

		if (nums[right]+len(q))%2 == 0 {
			if right+k > len(nums) {
				return -1
			}
			res++
			q = append(q, right)

		}
		right++
	}
	return res
}
