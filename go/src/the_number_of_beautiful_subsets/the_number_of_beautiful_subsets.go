package leetcode

func abs(num int)int{
	if num < 0{
		return num * -1
	}
	return num
}
func beautifulSubsets(nums []int, k int) int {
    curMap := make(map[int]int)

	return dfs(0, &nums, &curMap, k) -1
}
func dfs(idx int, nums *[]int, curMap *map[int]int, target int)int{
	if idx >= len(*nums){
		return 1
	}
	pos := (*nums)[idx] + target
	neg := (*nums)[idx] - target
	res := dfs(idx +1, nums, curMap, target)
	posVal, containsPos := (*curMap)[pos]
	negVal, containsNeg := (*curMap)[neg]
	if containsPos && posVal >0 || containsNeg && negVal > 0{
		return res
	}
	(*curMap)[(*nums)[idx]] += 1
	res += dfs(idx +1, nums, curMap, target)
	(*curMap)[(*nums)[idx]] -= 1
	return res
}
