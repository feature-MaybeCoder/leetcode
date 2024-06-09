package leetcode

func subarraysDivByK(nums []int, k int) int {
	prefix := make(map[int]int, k)
	sum := 0
	res := 0
	prefix[0] = 1
	for _, num := range nums {
		sum += num
		remainder := sum % k
        if remainder < 0{
            remainder += k
        }
		res += prefix[remainder]
        prefix[remainder] += 1

	}
	return res
}
