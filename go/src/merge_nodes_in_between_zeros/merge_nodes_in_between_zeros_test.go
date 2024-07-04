package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 *Node
		
        want int
    }{
        {"base case", nil, 0},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := mergeNodes(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("merge_nodes_in_between_zeros(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }

