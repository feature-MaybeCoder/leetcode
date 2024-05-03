package palindromicSubstrings

func isPalindrom(s *string) bool {
	start := 0
	end := len(s)-1
	for start < end {
		if s[start] != s[end] {
			return false
		}
		start += 1
		end -= 1
	}
	return true
}
func countSubstrings(s string) int {
	num := 0

	for i := 0; i < len(s); i++ {
		s_l := len(s) - i
		for j := 0; j + s_l <= len(s); j++ {
			if (isPalindrom(&s[j:j+s_l])){
				num+=1
			}
		}
	}

	return num
}
