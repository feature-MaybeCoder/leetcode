package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 int
		
        want string
    }{
        {"base case", 123, "One Hundred Twenty Three"},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := numberToWords(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("integer_to_english_words(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }

