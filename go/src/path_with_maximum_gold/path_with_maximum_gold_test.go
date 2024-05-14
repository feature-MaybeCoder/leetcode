package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 [][]int
		
        want int
    }{
        {"base case", [][]int{{1,0,7},{2,0,6},{3,4,5},{0,3,0},{9,0,20}}, 28},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := getMaximumGold(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("path_with_maximum_gold(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }

