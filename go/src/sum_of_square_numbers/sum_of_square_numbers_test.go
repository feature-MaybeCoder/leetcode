package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 int

		want bool
	}{
		{"base case", 5, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := judgeSquareSum(tt.arg1)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("sum_of_square_numbers(%v) = %v, want %v", tt.arg1, got, tt.want)
			}
		})
	}
}
