package leetcode

type Uf struct {
	childs []int
}

func find(this *Uf, node int) int {
	for node != this.childs[node] {
		this.childs[node] = this.childs[this.childs[node]]
		node = this.childs[node]
	}
	return node
}
func union(this *Uf, node1 int, node2 int) bool {
	p1 := find(this, node1)
	p2 := find(this, node2)

	if p1 == p2 || p2 != node2{
		return false
	}
	this.childs[p2] = p1
	return true
}
func validateBinaryTreeNodes(n int, leftChild []int, rightChild []int) bool {
	childs := make([]int, n)
	for i := 0; i < n; i++ {
		childs[i] = i
	}
	uf := Uf{childs}

	for i := 0; i < len(leftChild); i++ {
		if leftChild[i] == -1{
			continue
		}
		if (!union(&uf, i, leftChild[i])){
			return false
		}
		n -=1
	}
	for i := 0; i < len(rightChild); i++ {
		if rightChild[i] == -1{
			continue
		}
		if (!union(&uf, i, rightChild[i])){
			return false
		}
		n -=1
	}
	
	return n ==1 
}
