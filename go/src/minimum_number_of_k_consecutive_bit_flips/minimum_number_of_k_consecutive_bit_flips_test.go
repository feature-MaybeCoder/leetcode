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
        {"base case", []int{0,0,0,1,0,1,1,0}, 3, 3},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := minKBitFlips(tt.arg1, tt.arg2)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("minimum_number_of_k_consecutive_bit_flips(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
            }
        })
    }
    }
