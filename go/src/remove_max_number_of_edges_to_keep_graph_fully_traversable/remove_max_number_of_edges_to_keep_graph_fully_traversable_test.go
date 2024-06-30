package leetcode

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
	tests := []struct {
		name string
		arg1 int
		arg2 [][]int

		want int
	}{
		{"base case", 3, [][]int{{1, 1, 2}, {2, 1, 2}, {3, 1, 2}, {1, 2, 3}, {2, 2, 3}, {3, 2, 3}, {3, 1, 3}}, 5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := maxNumEdgesToRemove(tt.arg1, tt.arg2)
			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("remove_max_number_of_edges_to_keep_graph_fully_traversable(%v, %v) = %v, want %v", tt.arg1, tt.arg2, got, tt.want)
			}
		})
	}
}
