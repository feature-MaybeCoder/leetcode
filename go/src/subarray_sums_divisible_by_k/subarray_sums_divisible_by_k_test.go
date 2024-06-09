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
        {"base case", []int{4,5,0,-2,-3,1}, 5, 7},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := subarraysDivByK(tt.arg1, tt.arg2)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("subarray_sums_divisible_by_k(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
            }
        })
    }
    }
