package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 string
		want int
	}{
		{"basic case", "cbdkb", 3},
		{"whole match", "bbbab", 4},
        {"odd chars case", "aaa", 3},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := longestPalindromeSubseq(tt.arg1)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("longestPalindromeSubseq(%v) = %v, want %v", tt.arg1, got, tt.want)
			}
		})
	}
}
