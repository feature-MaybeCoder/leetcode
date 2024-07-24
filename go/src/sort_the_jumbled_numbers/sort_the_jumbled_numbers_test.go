package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 []int
		arg2 []int
		
        want []int
    }{
        {"base case", []int{9,8,7,6,5,4,3,2,1,0}, []int{0,1,2,3,4,5,6,7,8,9}, []int{9,8,7,6,5,4,3,2,1,0}},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := sortJumbled(tt.arg1, tt.arg2)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("sort_the_jumbled_numbers(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
            }
        })
    }
    }

