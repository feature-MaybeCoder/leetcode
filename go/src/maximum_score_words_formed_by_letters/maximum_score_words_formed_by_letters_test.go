package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 []string
		arg2 []byte
		arg3 []int
		
        want int
    }{
        {"base case", []string{"dog","cat","dad","good"}, []byte{97, 97, 99, 100, 100, 100, 103, 111, 111}, []int{1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0}, 23},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := maxScoreWords(tt.arg1, tt.arg2, tt.arg3)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("maximum_score_words_formed_by_letters(%v, %v, %v) = %v, want %v", tt.arg1, tt.arg2, tt.arg3, got, tt.want)
            }
        })
    }
    }
