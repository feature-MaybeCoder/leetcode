package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 int
		arg2 []int
		arg3 []int
		
        want bool
    }{
        {"base case", 5, []int{1,-1,3,4,-1}, []int{2,-1,-1,-1, -1}, true},
        {"loop case", 4, []int{1,-1,3,-1}, []int{2,3,-1,-1}, false},
        {"one edge case", 3, []int{1,-1,-1}, []int{-1,-1,1}, false},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := validateBinaryTreeNodes(tt.arg1, tt.arg2, tt.arg3)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("validate_binary_tree_nodes(%v, %v, %v) = %v, want %v", tt.arg1, tt.arg2, tt.arg3, got, tt.want)
            }
        })
    }
    }

