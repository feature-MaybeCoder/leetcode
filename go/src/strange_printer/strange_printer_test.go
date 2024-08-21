package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 string

		want int
	}{
		{"base case", "abcbc", 4},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := strangePrinter(tt.arg1)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("strange_printer(%v) = %v, want %v", tt.arg1, got, tt.want)
			}
		})
	}
}
