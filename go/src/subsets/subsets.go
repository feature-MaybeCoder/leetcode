package leetcode
func subsets(nums []int) [][]int {
	res := make([][]int, 0)
	res = append(res, []int{})
	for i := 0; i < len(nums); i++ {
		rLen := len(res)
		for j := 0; j < rLen; j++ {
			
			tmp := make([]int, len(res[i]))
			copy(tmp, res[i])
			tmp = append(tmp, nums[i])
			res = append(res, tmp)
		}
	}
	return res
}