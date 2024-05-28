package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 string
		arg2 string
		arg3 int
		
        want int
    }{
        {"base case", "abcd", "bcdf", 3, 3},
        {"middle trim case", "aaaaagaa", "aaagaaaa", 3, 4},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := equalSubstring(tt.arg1, tt.arg2, tt.arg3)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("get_equal_substrings_within_budget(%v, %v, %v) = %v, want %v", tt.arg1, tt.arg2, tt.arg3, got, tt.want)
            }
        })
    }
    }
