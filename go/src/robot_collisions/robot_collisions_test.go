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
		arg3 string
		
        want int
    }{}

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := survivedRobotsHealths(tt.arg1, tt.arg2, tt.arg3)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("robot_collisions(%v, %v, %v) = %v, want %v", tt.arg1, tt.arg2, tt.arg3, got, tt.want)
            }
        })
    }
    }
