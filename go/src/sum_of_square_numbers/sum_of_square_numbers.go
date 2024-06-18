package leetcode

func judgeSquareSum(c int) bool {

	mapped := map[int]int{}
	for i := 0; i*i <= c; i++ {
		mapped[i*i] = 1
	}
	for i := 0; i*i <= c; i++ {
		diff := c - i*i
		_, ok := mapped[diff]
		if ok {
			return true
		}
	}
	return false
}
