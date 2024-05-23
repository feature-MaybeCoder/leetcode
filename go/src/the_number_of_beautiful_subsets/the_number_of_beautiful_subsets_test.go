package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 []int
		arg2 int
		
        want int
    }{
        {"base case", []int{2,4,6}, 2, 4},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := beautifulSubsets(tt.arg1, tt.arg2)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("the_number_of_beautiful_subsets(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
            }
        })
    }
    }

