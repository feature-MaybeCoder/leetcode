package leetcode
func Max(x, y int) int {
 if x > y {
   return x
 }
 return y
}
func dfs(start int, end int, s *string, cache *[][]int) int {
	if start > end {
		return 0
	}
	if start == end{
		return 1
	}
	if (*cache)[start][end] != -1 {
		return (*cache)[start][end]
	}
	
	if (*s)[start] == (*s)[end] {
		res := dfs(start+1, end-1, s, cache) +2
		(*cache)[start][end] = res
		return res
	}
	amount := 0
	
	left := dfs(start+1, end, s, cache)
	right := dfs(start, end-1, s, cache)

	if left > right {
		amount += left
	} else {
		amount += right
	}

	(*cache)[start][end] = amount
	return amount
}
func longestPalindromeSubseqDfs(s string) int {
	cache_len := len(s)
	cache := make([][]int, cache_len)
	for x := 0; x < cache_len; x++ {
		cache[x] = make([]int, cache_len)
		for y := 0; y < cache_len; y++ {
			cache[x][y] = -1
		}
	}

	return dfs(0, cache_len-1, &s, &cache)
}
func longestPalindromeSubseq2dDp(s string) int {
	cache_len := len(s)
	cache := make([][]int, cache_len)
	start := cache_len-1
	for x := start; x >= 0; x-- {
		cache[x] = make([]int, cache_len)
		cache[x][x] = 1
		for y := x+1; y < cache_len; y++ {
			if s[x] == s[y]{
				cache[x][y] = 2 + cache[x+1][y-1]
				continue
			}
			cache[x][y] = Max(cache[x+1][y], cache[x][y-1])
		}
	}
	return cache[0][cache_len-1]
}
func longestPalindromeSubseq(s string) int {
	cache_len := len(s)
	cache := make([]int, cache_len)
	start := cache_len-1
	for x := start; x >= 0; x-- {
		cache[x] = 1
		prev := 0
		for y := x+1; y < cache_len; y++ {
			temp := cache[y]
			if s[x] == s[y]{
				cache[y] = prev +2
			}else{
				cache[y] = Max(cache[y], cache[y-1])
			}
			prev = temp
		}
	}
	return cache[start]
}
