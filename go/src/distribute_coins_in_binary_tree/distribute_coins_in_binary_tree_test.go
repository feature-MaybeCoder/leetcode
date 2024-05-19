package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 *TreeNode

		want int
	}{
		{"base case", &TreeNode{
			Val: 0,
			Left: &TreeNode{
				Val:   3,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Val:   0,
				Left:  nil,
				Right: nil,
			},
		}, 3},
		{"reach root case", &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val:   0,
				Left:  nil,
				Right: nil,
			},
			Right: nil,
		}, 1},
		{"deep right tree", &TreeNode{
			Val: 0,
			Left: &TreeNode{
				Val:   3,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Val:  1,
				Left: nil,
				Right: &TreeNode{
					Val:  1,
					Left: nil,
					Right: &TreeNode{
						Val:   0,
						Left:  nil,
						Right: nil,
					},
				},
			},
		}, 5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := distributeCoins(tt.arg1)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("distributeCoins(%v) = %v, want %v", tt.arg1, got, tt.want)
			}
		})
	}
}
