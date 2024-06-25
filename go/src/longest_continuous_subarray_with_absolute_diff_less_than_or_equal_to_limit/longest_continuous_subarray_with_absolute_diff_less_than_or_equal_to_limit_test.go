package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 []int
		arg2 int

		want int
	}{
		{"base case", []int{8, 2, 4, 7}, 4, 2},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := longestSubarray(tt.arg1, tt.arg2)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("longestSubarray(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
			}
		})
	}
}
