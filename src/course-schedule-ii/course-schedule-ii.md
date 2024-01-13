

----

Tags: #graph #dp #medium #leetcode #breath-first-search

----

## Drawing:
[[course-schedule-ii.excalidraw]]

----


## solution explanation:
represent graph in more helpful data fromat, get graph roots that have <2 connections, make left traverse of graph, check on loop and visited cache

## last submission:
```javascript
/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @return {number[]}
 */
var findOrder = function (numCourses, prerequisites) {
    if (numCourses === 0) return []
    let i = numCourses
    if (!prerequisites.length) return Array.from({ length: numCourses }, () => {
        i--
        return i
    })
    const graph = {}
    const dependencygraph = {}
    const res = []
    let amount = 0
    for (let [from, to] of prerequisites) {
        if (!(from in graph)) {
            dependencygraph[from] = []
            graph[from] = []
            amount++
        }
        if (!(to in graph)) {
            dependencygraph[to] = []
            graph[to] = []
            amount++
        }
        dependencygraph[to].push(from)
        graph[from].push(to)
        if (amount > numCourses) return res
    }
    let root = []
    
    for (let i = 0; i < numCourses; i++) {
        const connections = dependencygraph[i]
        
        if (connections === undefined) {
            res.push(i)
            continue
        }
        if (connections.length < 2) {
            root.push(i)
        }
    }
    if (root.length === 0) {
        return res
    }

    const dfs = (nodes, visitedLoop, visitedCache) => {
        
        if (nodes in visitedLoop) {
            return true
        }
        if (nodes in visitedCache) return false
        visitedCache[nodes] = true
        visitedLoop[nodes] = true
        for (const connection of graph[nodes]) {
            const isLoop = dfs(connection, visitedLoop, visitedCache)
            if (isLoop) return true
        }

        res.push(nodes)
        delete visitedLoop[nodes]
        return false
    }
    const visitedCache = {}
    const loopCache = {}
    for (const _root of root) {

        const isLoop = dfs(_root, loopCache, visitedCache)
        if (isLoop) return []
    }
    return res
};
```



