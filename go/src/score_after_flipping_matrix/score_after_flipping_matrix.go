package leetcode
func matrixScore(grid [][]int) int {
	height := len(grid)
	width := len(grid[0])
	for x := 0; x < height; x++ {
		if grid[x][0] == 0 {
			for y := 0; y < width; y++ {
				if grid[x][y] == 1 {
					grid[x][y] = 0
					continue
				}
				grid[x][y] = 1
			}
		}
	}
	for y := 0; y < width; y++ {
		ones := 0
		zeros := 0
		for x := 0; x < height; x++ {
			if grid[x][y] == 1 {
				ones += 1
				continue
			}
			zeros += 1

		}
		if zeros > ones {
			for x := 0; x < height; x++ {
				if grid[x][y] == 1 {
					grid[x][y] = 0
					continue
				}
				grid[x][y] = 1
			}
		}
	}
	res := 0
	for x := 0; x < height; x++ {
		temp := 0
		for y := 0; y < width; y++ {
			temp |= grid[x][y]
			if y < width -1{
				temp <<= 1
			}
			
		}
		res+=temp
	}
	return res
}
