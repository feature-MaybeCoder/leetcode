package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 string
		
        want string
    }{
        {"base case", "Mg(OH)2", "H2MgO2"},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := countOfAtoms(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("number_of_atoms(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }
