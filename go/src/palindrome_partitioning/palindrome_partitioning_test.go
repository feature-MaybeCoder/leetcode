package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 string
		
        want [][]string
    }{
        {"base case", "aab", [][]string{{"a","a","b"}, {"aa", "b"}}},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := partition(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("palindrome_partitioning(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }

