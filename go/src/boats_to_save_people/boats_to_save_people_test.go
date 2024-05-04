package boats_to_save_people

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 []int
		arg2 int
		
        want int
    }{
        {"base case", {3,2,2,1,}, 3, 3},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := numRescueBoats(tt.arg1, tt.arg2)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("numRescueBoats(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
            }
        })
    }
    }
