package palindromic_substrings

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        inpt string
        want int
    }{
        {"basic case", "abc",3},
        {"all palindrom case", "aaa", 6},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := countSubstrings(tt.inpt)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("isPalindrom(%v) = %v, want %v", tt.inpt,got, tt.want)
            }
        })
    }
    }

