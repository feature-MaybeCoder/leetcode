package leetcode

func checkSubarraySum(nums []int, k int) bool {
	numsL := len(nums)
	mapped := make(map[int]int, numsL)
	mapped[0] = -1
	sum := 0
	for i := 0; i < numsL; i++ {
		sum += nums[i]
		remainder := sum % k
		val, ok := mapped[remainder]
		if ok {
			if i-val > 1 {
				return true
			}
			continue
		}
		mapped[remainder] = i
	}
	return false
}
