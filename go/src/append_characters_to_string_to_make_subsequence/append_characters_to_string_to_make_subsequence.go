package leetcode

// two pointers solution
func appendCharacters(s string, t string) int {
	height := len(s)
	width := len(t)
	left := 0
	right := 0
	for left < height && right < width{
		if s[left] == t[right]{
			left +=1
			right+=1
			continue
		}
		left+=1
	}
	return width - right
}


// lcs solution

type Cache = [][]int

func max(int1 int, int2 int) int {
	if int1 > int2 {
		return int1
	}
	return int2
}
func dfs(x int, y int, s *string, t *string, cache *Cache) int {
 	if y >= len(*t) {
		return 1
	}
	if x >= len(*s) {
		return 0
	}
	if (*cache)[x][y] != -1 {
		return (*cache)[x][y]
	}
	res := dfs(x+1, y, s, t, cache)
	if (*s)[x] == (*t)[y] {
		res = max(res, dfs(x+1, y+1, s, t, cache) + 1)
	}
	(*cache)[x][y] = res
	return res
}
