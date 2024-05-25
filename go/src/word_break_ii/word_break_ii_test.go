package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 string
		arg2 []string
		
        want []string
    }{
        {"base case", "catsanddog", []string{"cat","cats","and","sand","dog"}, []string{"cats and dog","cat sand dog"}},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := wordBreak(tt.arg1, tt.arg2)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("word_break_ii(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
            }
        })
    }
    }

