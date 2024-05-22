package leetcode
func partition(s string) [][]string {
	res := make([][]string, 0)
	cur :=make([]string, 0)
	dfs(0, &s, &cur, &res)

	return res
}
func isPalindrom(str string)bool{
	start := 0
	end := len(str)-1

	for start < end{
		if str[start] != str[end]{
			return false
		}
		start +=1
		end -=1
	}
	return true
}
func dfs(idx int, str *string, cur *[]string, res *[][]string){
	if idx >= len(*str){
		cloned := make([]string, len(*cur))
		copy(cloned, *cur)
		*res = append(*res, cloned)
		return
	}
	for i := idx; i < len(*str); i++ {
		slice := (*str)[idx:i+1]
		if (!isPalindrom(slice)){
			continue
		}
		*cur = append(*cur, slice)
		dfs(i+1, str, cur, res)
		*cur = (*cur)[:len(*cur)-1]
	}
}