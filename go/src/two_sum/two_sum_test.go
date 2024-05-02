package two_sum

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        nums   []int
        target int
        want   []int
    }{
        {"basic case", []int{2, 7, 11, 15}, 9, []int{0, 1}},
    }

    for _, tt := range tests {

        t.Run(tt.name, func(t *testing.T) {
            got := twoSum(tt.nums, tt.target)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("twoSum(%v, %v) = %v, want %v", tt.nums, tt.target, got, tt.want)
            }
        })
    }
    }
