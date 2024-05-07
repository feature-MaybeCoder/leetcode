package double_a_number_represented_as_a_linked_list

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 *ListNode
		
        want *ListNode
    }{
        {"base case", &ListNode{1, nil}, &ListNode{2, nil}},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := doubleIt(tt.arg1)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("double_a_number_represented_as_a_linked_list(%v) = %v, want %v", tt.arg1, got, tt.want)
            }
        })
    }
    }

