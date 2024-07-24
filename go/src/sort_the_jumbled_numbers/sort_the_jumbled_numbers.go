package leetcode

import (
	"runtime/debug"
	"sort"
)
type Item struct{
    original int
    mapped int
    idx int
}
func sortJumbled(mapping []int, nums []int) []int {
    toSort := make([]Item, len(nums))
    for i, n := range nums{
        toSort[i] = Item{
            original: n,
            mapped: replaceNum(mapping, n),
            idx: i,
        }
    }

    sort.Slice(toSort, func(i, j int)bool{
        if toSort[i].mapped == toSort[j].mapped{
            return toSort[i].idx < toSort[j].idx
        }
        return toSort[i].mapped < toSort[j].mapped
    })
    for i, n := range toSort{
        nums[i] = n.original
    }
    return nums
}

func replaceNum(mapping []int, n int)int{
    mul := 1
    res := 0
    if n == 0{
        return mapping[0]
    }
    for n > 0{
        nth := n % 10
        res += mapping[nth] * mul
        n = n /10
        mul *= 10
    }
    return res
}
func init(){
    debug.SetGCPercent(-1)
}