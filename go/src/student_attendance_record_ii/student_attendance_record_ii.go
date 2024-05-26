package leetcode

type Cache = [][2][3]int
func checkRecord(n int) int {
	cache := make(Cache, n)
	mod := 1000000007;
    return dfs(0, 1, 2, n, mod, &cache)
}
func min(num1 int, num2 int)int{
	if num1 > num2{
		return num2
	}
	return num1
}
func dfs(idx int, abs int, late int, len int, mod int, cache *Cache)int{
	if idx == len {
		return 1
	}
	val := (*cache)[idx][abs][late]
	if val != 0{
		return val
	}
	res := dfs(idx +1, abs, 2, len, mod, cache)
	if abs > 0{
		res += dfs(idx +1, 0, 2, len, mod, cache)
	}
	if late > 0{
		res += dfs(idx +1, abs, late-1, len, mod, cache)
	}
	res = res % mod
	(*cache)[idx][abs][late] = res
	return res
}
