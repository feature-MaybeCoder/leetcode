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
        {"base case", [][]int{{0,1,1,0},{0,1,1,0},{0,0,0,0}}, 2},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := minDays(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("minimum_number_of_days_to_disconnect_island(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }
