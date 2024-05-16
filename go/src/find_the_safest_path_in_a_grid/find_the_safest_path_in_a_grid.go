package leetcode

import (
	"container/heap"
	"math"
)

func Abs(num int) int {
	if num < 0 {
		return num * -1
	}
	return num
}
func Max(num int, num2 int) int {
	if num > num2 {
		return num
	}
	return num2
}
func Min(num int, num2 int) int {
	if num < num2 {
		return num
	}
	return num2
}
func calcManhattan(x1 int, x2 int, y1 int, y2 int) int {
	return Abs(x1-x2) + Abs(y1-y2)
}

type Item struct {
	x        int
	y        int
	priority int // The priority of the item in the queue.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].priority > pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x any) {
	item := x.(*Item)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil // avoid memory leak
	*pq = old[0 : n-1]
	return item
}

type Node struct {
	x  int
	y  int
	ox int
	oy int
}

func maximumSafenessFactor(grid [][]int) int {
	width := len(grid[0])
	height := len(grid)
	safeness := make([][]int, height)
	dirs := []int{0, 1, 0, -1, 0}
	stack := make([]Node, 0)
	for x := 0; x < height; x++ {
		safeness[x] = make([]int, width)
		for y := 0; y < width; y++ {
			safeness[x][y] = math.MaxInt
			if grid[x][y] == 1 {
				stack = append(stack, Node{x, y, x, y})
				safeness[x][y] = 0
			}
		}

	}
	if safeness[0][0] == 0 {
		return 0
	}
	for len(stack) > 0 {
		newStack := make([]Node, 0, len(grid))
		for i := 0; i < len(stack); i++ {
			node := stack[i]
			for i := 0; i < 4; i++ {
				dx := node.x + dirs[i]
				dy := node.y + dirs[i+1]
				if dx < 0 || dy < 0 || dx >= len(grid) || dy >= len(grid[0]) {
					continue
				}
				if grid[dx][dy] == 1 {
					continue
				}
				mht := calcManhattan(node.ox, dx, node.oy, dy)
				if mht >= safeness[dx][dy] {
					continue
				}
				safeness[dx][dy] = mht
				newStack = append(newStack, Node{dx, dy, node.ox, node.oy})
			}

		}
		stack = newStack
	}
	pq := make(PriorityQueue, 0, height)

	heap.Init(&pq)
	heap.Push(&pq, &Item{0, 0, safeness[0][0]})

	for pq.Len() > 0 {
		node := heap.Pop(&pq).(*Item)
		if node.x == height-1 && node.y == width-1 {
			return node.priority
		}
		for i := 0; i < 4; i++ {
			dx := node.x + dirs[i]
			dy := node.y + dirs[i+1]
			if dx < 0 || dy < 0 || dx >= height || dy >= width || grid[dx][dy] == -1 {
				continue
			}
			grid[dx][dy] = -1
			heap.Push(&pq, &Item{x: dx, y: dy, priority: Min(node.priority, safeness[dx][dy])})
		}
	}
	return -1
}
