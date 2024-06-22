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
        {"base case", []int{2,2,2,1,2,2,1,2,2,2}, 2, 16},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := numberOfSubarrays(tt.arg1, tt.arg2)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("count_number_of_nice_subarrays(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
            }
        })
    }
    }
