package leetcode

import (
	"runtime/debug"
	"sort"
)

type Node struct{
    pos int
    idx int
}
func survivedRobotsHealths(positions []int, healths []int, directions string) []int {
    res := make([]int, 0, len(positions))
    robots := make([]Node, len(positions))
    stack := make([]Node, 0, len(positions))
    for idx, pos := range positions{
        robots[idx] = Node{
            pos: pos,
            idx: idx,
        }
    }

    sort.Slice(robots, func(i, j int) bool {
		return robots[i].pos < robots[j].pos
	})
    for _, r := range robots{
        if rune(directions[r.idx]) == 'R'{
            stack = append(stack, r)
            continue
        }
        for len(stack)>0{
            last := stack[len(stack) -1]
            if healths[r.idx] == healths[last.idx]{
                stack = stack[:len(stack)-1]
                healths[r.idx] = -1
                healths[last.idx] = -1
                break
            }
            if healths[r.idx] > healths[last.idx]{
                healths[r.idx]-=1
                healths[last.idx] = -1
                stack = stack[:len(stack)-1]
                continue
            }
            healths[last.idx] -= 1
            healths[r.idx] = -1
            break
        }   

    }

    for _, h := range healths{
        if h > 0{
            res = append(res, h)
        }
    }


    return res
}
func init(){
    debug.SetGCPercent(-1)
}
