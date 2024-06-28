package leetcode

import "sort"

func maximumImportance(n int, roads [][]int) int64 {
    adj := make([][2]int, n)
    w := make([]int64, n)
    for i := 0; i <n; i++{
        adj[i] = [2]int{i, 0}
    }
    for _, r := range roads{
        adj[r[0]][1] +=1
        adj[r[1]][1] +=1
    }

    sort.Slice(adj, func(i,j int)bool{
        return adj[i][1] > adj[j][1]
    })
    for i := 0; i < n; i++{
        w[adj[i][0]] = int64(n - i)
    }
    var res int64 = 0
    for _, r := range roads{
        res += w[r[0]]
        res += w[r[1]]
    }

    return res
}
