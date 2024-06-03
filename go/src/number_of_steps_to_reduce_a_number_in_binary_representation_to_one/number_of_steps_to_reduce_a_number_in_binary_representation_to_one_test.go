package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 string
		
        want int
    }{
        {"base case", "1101", 6},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := numSteps(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("number_of_steps_to_reduce_a_number_in_binary_representation_to_one(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }
