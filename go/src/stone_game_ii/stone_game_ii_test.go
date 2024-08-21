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
		{"base case", []int{2, 7, 9, 4, 4}, 10},
		{"case", []int{2, 7, 9, 100, 100, 100, 100, 100}, 211},
		{"case", []int{1}, 1},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := stoneGameII(tt.arg1)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("stone_game_ii(%v) = %v, want %v", tt.arg1, got, tt.want)
			}
		})
	}
}
