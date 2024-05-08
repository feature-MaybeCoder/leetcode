package get_watched_videos_by_your_friends

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        arg1 [][]string
		arg2 [][]int
		arg3 int
		arg4 int
		
        want []string
    }{
        {"base case", [][]string{{"A", "B"},{"C"},{"B","C"},{"D"}}, [][]int{{1,2},{0,3},{0,3},{1,2}}, 0, 2, []string{"D"}},
        {"frequency order", [][]string{{"A", "B"},{"C"},{"B","C"},{"D"}}, [][]int{{1,2},{0,3},{0,3},{1,2}}, 0, 1, []string{"B", "C"}},
        {"alph order", [][]string{{"xk", "qvgjjsp", "sbphxzm"},{"rwyvxl", "ov"}}, [][]int{{1},{0}}, 0, 1, []string{"ov", "rwyvxl"}},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := watchedVideosByFriends(tt.arg1, tt.arg2, tt.arg3, tt.arg4)
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("get_watched_videos_by_your_friends(%v, %v, %v, %v) = %v, want %v", tt.arg1, tt.arg2, tt.arg3, tt.arg4, got, tt.want)
            }
        })
    }
    }

