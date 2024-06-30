package leetcode

import "runtime/debug"

type Uf struct {
    p []int
    merged int
}

func (this *Uf)Union(n1, n2 int)int{
    p1 := this.Find(n1)
    p2 := this.Find(n2)
    if p1 == p2{
        return 1
    }
    this.merged--
    this.p[p1] = p2
    return 0
}

func (this *Uf)Find(n int)int{
    for this.p[n] != n{
        this.p[n] = this.p[this.p[n]]
        n = this.p[n]
    }
    return n
}
func New(n int)Uf{
    p := make([]int, n)

    for i := 0; i < n; i++{
        p[i] = i
    }

    return Uf{
        p: p,
        merged: n -1,
    }
}
func maxNumEdgesToRemove(n int, edges [][]int) int {
    r, g := New(n), New(n)
    res := 0
    for _, e := range edges{
        from := e[1] -1
        to := e[2] -1
        if e[0] == 3{
            rs := r.Union(from, to)
            rg := g.Union(from, to)
            if rs == 1 && rg == 1{
                res++
            }
        }
    }
    for _, e := range edges{
        from := e[1] -1
        to := e[2] -1
        if e[0] == 1{
            res += r.Union(from, to)
        }
        if e[0] == 2{
            res += g.Union(from, to)
        }
    }
    if r.merged > 0 || g.merged > 0{
        return -1
    }
    return res
}

func init() {
    debug.SetGCPercent(-1)
}
