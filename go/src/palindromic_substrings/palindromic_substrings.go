package leetcode
func countSubstrings(s string) int{
	sLen := len(s)
	// iach charateer is palindrom so we init counter with len
	amount := sLen

	for i := 0; i < sLen; i++ {
		// we dont need to check each charecter so we can safe one iteration on each step
		left := i-1;
		right := i+1;
		for left >= 0 && right < sLen	&& s[left] == s[right]{
			amount+=1;
			left-=1
			right+=1
		}
		left = i
		right = i +1
		for left >= 0 && right < sLen && s[left] == s[right]{
			amount+=1
			left-=1
			right+=1

		}
	}

	return amount
}

