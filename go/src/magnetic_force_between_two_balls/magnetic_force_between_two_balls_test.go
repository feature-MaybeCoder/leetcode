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
		{"base case", []int{1, 2, 3, 4, 7}, 3, 3},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := maxDistance(tt.arg1, tt.arg2)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("magnetic_force_between_two_balls(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
			}
		})
	}
}
