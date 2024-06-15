package leetcode

import (
	"container/heap"
	"sort"
)

type Node struct {
	profit  int
	capital int
}

type PHeap []int

func (h PHeap) Len() int           { return len(h) }
func (h PHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h PHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *PHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *PHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func findMaximizedCapital(k int, w int, profits []int, capital []int) int {
	pLen := len(profits)
	adj := make([]Node, pLen)
	for idx, cap := range capital {
		adj[idx] = Node{
			profit:  profits[idx],
			capital: cap,
		}
	}
	sort.Slice(adj, func(i, j int) bool {
		return adj[i].capital < adj[j].capital
	})
	hp := &PHeap{}
	heap.Init(hp)
	idx := 0
	for k > 0  {
		for idx < pLen && adj[idx].capital <= w {
			heap.Push(hp, adj[idx].profit)
			idx += 1
		}
		if len(*hp) == 0 {
			break
		}
		w += heap.Pop(hp).(int)
		k -= 1
	}

	return w
}
