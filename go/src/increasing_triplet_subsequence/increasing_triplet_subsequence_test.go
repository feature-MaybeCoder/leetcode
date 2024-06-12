package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 []int
		
        want bool
    }{
        {"base case", []int{1,2,3,4,5}, true},
        {"base falsy case", []int{5,4,3,2,1}, false},
        {"interval case", []int{4,1,5,6}, true},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := increasingTriplet(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("increasing_triplet_subsequence(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }
