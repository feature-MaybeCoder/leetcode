package leetcode
func dfs(x int, y int, grid *[][]int, dirs *[]int) int{
	tmp := (*grid)[x][y]
	sum := tmp
	max := 0
	(*grid)[x][y] = 0

	for i := 0; i < 4; i++ {
		dx := x+(*dirs)[i]
		dy := y+(*dirs)[i+1]
		if dx < 0 || dy < 0 || dx >= len(*grid) || dy >= len((*grid)[0]) || (*grid)[dx][dy]==0{
			continue
		}
		tmp := dfs(dx,dy,grid,dirs)
		if tmp > max{
			max = tmp
		}
	}

	(*grid)[x][y] = tmp
	return max + sum
}
func getMaximumGold(grid [][]int) int {
	width := len(grid[0])
	height := len(grid)
	dirs := []int{0,1,0,-1,0}
	visited := make([][]int, height)
	for x := 0; x < height; x++ {
		visited[x] = make([]int, width)	
		copy(visited[x], grid[x])
	}
	max := 0
	for x := 0; x < height; x++ {
		for y := 0; y < width; y++ {
			if visited[x][y] == 0{
				continue
			}
			tmp := dfs(x,y,&grid, &dirs)
			if tmp > max{
				max = tmp
			}
			visited[x][y] = 0
		}
	}
	return max
}

