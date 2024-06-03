package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 int
		
        want int
    }{
        {"base case", 0, 0},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := single_number_iii(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("single_number_iii(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }

