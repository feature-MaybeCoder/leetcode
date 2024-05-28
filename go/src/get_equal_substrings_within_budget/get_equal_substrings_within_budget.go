package leetcode

func abs(num int) int {
	if num < 0 {
		return num * -1
	}
	return num
}
func max(num int, num2 int) int {
	if num > num2 {
		return num
	}
	return num2
}
func equalSubstring(s string, t string, maxCost int) int {
	res := 0
	sLen := len(s)
	costs := make([]int, sLen)
	shift := int(byte('a'))
	for i := 0; i < sLen; i++ {
		sByte := int(s[i]) - shift
		tByte := int(t[i]) - shift
		costs[i] = abs(sByte - tByte)
	}
	sum := 0
	left := 0
	right := 0
	for right < sLen{
		if sum <= maxCost{
			res = max(res, right - left)
			sum += costs[right]
			right += 1
			continue
		}
		for sum > maxCost && left <= right{
			sum -= costs[left]
			left += 1
		}
	}
	for sum > maxCost && left <= right{
		sum -= costs[left]
		left += 1
	}
	res = max(res, right - left)
	return res
}
