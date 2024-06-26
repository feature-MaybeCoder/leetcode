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
	}{}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := balanceBST(tt.arg1)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("balance_a_binary_search_tree(%v) = %v, want %v", tt.arg1, got, tt.want)
			}
		})
	}
}
