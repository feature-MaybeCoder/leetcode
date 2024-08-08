package leetcode
func spiralMatrixIII(rows int, cols int, rStart int, cStart int) [][]int {
    res := make([][]int, 0, rows * cols)
    res = append(res, []int{rStart, cStart})
    ampl := 1
    for true {
        top := rStart - ampl
        bottom := rStart + ampl
        left := cStart - ampl
        right := cStart + ampl
        if top < 0 && bottom >= rows && left < 0 && right >= cols{
            break
        }
        if right < cols{
            m := min(bottom, rows-1)
            for x := max(top+1, 0); x <= m; x++{
                res = append(res, []int{x, right})
            }
        }
        if bottom < rows{
            m := max(0, left)
            for y := min(right-1, cols-1); y >= m; y--{
                res = append(res, []int{bottom, y})
            }
        }

        if left >= 0{
            m := max(0, top)
            for x := min(bottom-1, rows-1); x >= m; x--{
                res = append(res, []int{x, left})
            }
        }

        if top >= 0{
            m :=  min(right, cols-1)
            for y := max(left +1, 0); y <=m; y++{
                res = append(res, []int{top, y})
            }
        }

        ampl++
    }


    return res
}

func min(n1, n2 int)int{
    if n1 < n2{
        return n1
    }
    return n2
}

func max(n1, n2 int)int{
    if n1 > n2{
        return n1
    }
    return n2
}
