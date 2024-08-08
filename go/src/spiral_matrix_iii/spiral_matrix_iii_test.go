package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 int
		arg2 int
		arg3 int
		arg4 int

		want [][]int
	}{
		{"base case", 1, 4, 0, 0, [][]int{{0, 0}, {0, 1}, {0, 2}, {0, 3}}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := spiralMatrixIII(tt.arg1, tt.arg2, tt.arg3, tt.arg4)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("spiral_matrix_iii(%v, %v, %v, %v) = %v, want %v", tt.arg1, tt.arg2, tt.arg3, tt.arg4, got, tt.want)
			}
		})
	}
}
