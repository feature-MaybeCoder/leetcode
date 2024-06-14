package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 []int

		want int
	}{
		{"base case", []int{1, 2, 2}, 1},
		{"tripple increment case", []int{3, 2, 1, 2, 1, 7}, 6},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := minIncrementForUnique(tt.arg1)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("minIncrementForUnique(%v) = %v, want %v", tt.arg1, got, tt.want)
			}
		})
	}
}
