package leetcode
func twoSum(nums []int, target int) []int {
    var m = make(map[int]int, len(nums))
    for idx, val := range nums{
        idx_prev, contains := m[val]
        if contains {
            return []int{idx_prev, idx}
        }
        want := target- val
        m[want] = idx
    }

    return []int{}
}
