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
        {"base case", 2, 8},
        {"large case", 100, 183236316},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := checkRecord(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("student_attendance_record_ii(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }
