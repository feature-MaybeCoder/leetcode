package largest_positive_integer_that_exists_with_its_negative
func findMaxK(nums []int) int {
    pos := make(map[int]bool, len(nums))
	max := -1
	for _, num := range nums{
		op := num*-1
		if (num > 0){
			if num <= max{
				continue
			}
			_, contains := pos[op]
			if contains{
				max = num
				continue
			}
			pos[num] = true
			continue
		}
		if op <= max{
			continue
		}
		_, contains := pos[op]
		if contains{
			max = op
			continue
		}
		pos[num] = true
	}

	return max
}