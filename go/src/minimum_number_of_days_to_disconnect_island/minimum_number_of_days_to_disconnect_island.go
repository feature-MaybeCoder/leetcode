package leetcode


func minDays(grid [][]int) int {
    dirs := [5]int{0,1,0,-1,0}
    h := len(grid)
    w := len(grid[0])
    var dfs func(x, y, target, to int)
    dfs = func (x, y, target, to int){
        if x < 0 || y < 0 || x >= h || y >= w || grid[x][y]!=target{
            return
        }
        grid[x][y]=to
        for i := 0; i < 4;i++{
            dfs(x+dirs[i], y + dirs[i+1], target, to)
        }
    }
    var am func(target, to int)int
    am = func(target, to int)int {
        c := 0
        for x := 0; x < h; x++{
            for y := 0; y < w; y++{
                if grid[x][y]!=target{
                    continue
                }
                c++
                dfs(x,y, target, to)
            }
        }
        return c
    }
    if am(1,2) != 1{
        return 0
    }
    target, to := 2, 1
    for x := 0; x < h; x++{
        for y := 0; y < w; y++{
            if grid[x][y]==0{
                continue
            }
            grid[x][y] = to
            cur := am(target, to)
            if cur != 1{
                return 1
            }
            target, to = to, target
        }
    }
    return 2
}

