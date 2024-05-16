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
	index    int // The index of the item in the heap.
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
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}
type Node struct{
	x int
	y int
}
func bfs(x int, y int, grid *[][]int, safeness *[][]int, dirs *[]int) {
	stack := make([]Node, 0,len(*grid))
	stack = append(stack, Node{x,y})
	for len(stack) > 0{
		newStack := make([]Node, 0, len(*grid))
		for i := 0; i < len(stack); i++ {
			node := stack[i]
			for i := 0; i < 4; i++ {
				dx := node.x + (*dirs)[i]
				dy := node.y + (*dirs)[i+1]
				if dx < 0 || dy < 0 || dx >= len(*grid) || dy >= len((*grid)[0]){
					continue
				}
				if (*grid)[dx][dy] == 1{
					continue
				}
				mht := calcManhattan(x, dx, y, dy)
				if mht >= (*safeness)[dx][dy]{
					continue
				}
				(*safeness)[dx][dy] = mht
				newStack = append(newStack, Node{dx,dy})
			}
		
		}
		stack = newStack
	}
}
func maximumSafenessFactor(grid [][]int) int {
	width := len(grid[0])
	height := len(grid)
	safeness := make([][]int, height)
	dirs := []int{0, 1, 0, -1, 0}
	bfses := make([]Node, 0)
	for x := 0; x < height; x++ {
		safeness[x] = make([]int, width)
		for y := 0; y < width; y++ {
			safeness[x][y] = math.MaxInt
			if grid[x][y] == 1 {
				bfses = append(bfses, Node{x,y})
				safeness[x][y] = 0
			}
		}

	}
	if safeness[0][0] == 0 {
		return 0
	}
	for i := 0; i < len(bfses); i++ {
		bfs(bfses[i].x,bfses[i].y, &grid, &safeness, &dirs)
	}
	pq := make(PriorityQueue, 0, height)

	heap.Init(&pq)
	heap.Push(&pq, &Item{0, 0, safeness[0][0], 0})
	
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
