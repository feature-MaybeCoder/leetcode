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
		
        want int
    }{
        {"base case", "coaching", "coding", 4},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := appendCharacters(tt.arg1, tt.arg2)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("append_characters_to_string_to_make_subsequence(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
            }
        })
    }
    }
