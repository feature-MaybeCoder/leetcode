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

		want bool
	}{
		{"base case", []int{23, 2, 4, 6, 7}, 0, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := checkSubarraySum(tt.arg1, tt.arg2)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("checkSubarraySum(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
			}
		})
	}
}
