package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 []int
		arg2 []int
		arg3 []int

		want int
	}{
		{"base case", []int{2, 4, 6, 8, 10}, []int{10, 20, 30, 40, 50}, []int{4, 5, 6, 7}, 100},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := maxProfitAssignment(tt.arg1, tt.arg2, tt.arg3)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("most_profit_assigning_work(%v, %v, %v) = %v, want %v", tt.arg1, tt.arg2, tt.arg3, got, tt.want)
			}
		})
	}
}
