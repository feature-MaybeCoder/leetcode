package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 []int

		want []int
	}{
		{"base case", []int{2, 0, 2, 1, 1, 0}, []int{0, 0, 1, 1, 2, 2}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			sortColors(tt.arg1)
			if !reflect.DeepEqual(tt.arg1, tt.want) {
				t.Errorf("sort_colors(%v) = %v, want %v", tt.arg1, tt.arg1, tt.want)
			}
		})
	}
}
