

----

Tags: #graph #leetcode #medium #depth-first-search #union-find #repeat 

----

## Drawing:
[[satisfiability-of-equality-equations.excalidraw]]

----


## dfs solution explanation:
start for all possible roots, bipartite graph on two groups base on relation to current root for connection operation (to explain the calculation of the operation, see the figure),slightly modified bipartition dfs
## union find solution explanation:
iterate over all equations, union all equal equations, then find all nonequal equations only if each node have record in parents map, if nonequal operation have the same parent, that means we reach bad equation

## dfs submission:
```javascript
/**
 * @param {string[]} equations
 * @return {boolean}
 */
var equationsPossible = function (equations) {

    const graph = {}
    const uniqueNodes = []
    const equalSet = new Set()
    const nonEqualSet = new Set()
    for (let connection of equations) {
        const from = connection[0]
        const to = connection[3]
        const isEqual = connection[1] === '='
        if (from === to) {
            if (!isEqual) return false
            continue
        }
        if (!(from in graph)) {
            uniqueNodes.push(from)
            graph[from] = []
        }
        graph[from].push([to, isEqual])



    }


    function dfs(node, isEqual) {

        if (isEqual && nonEqualSet.has(node)) {
            return false
        }
        if (!isEqual && equalSet.has(node)) {
            return false
        }
        if (nonEqualSet.has(node) || equalSet.has(node)) return true
        if (isEqual) {
            equalSet.add(node)
        } else {
            nonEqualSet.add(node)
        }

        if (!graph[node]) {
            return true
        }
        for (let [to, operation] of graph[node]) {
            if (operation) {
                if (isEqual) {

                } else {
                    operation = false
                }
            } else {
                if (isEqual) {
                    operation = false
                } else {
                    continue
                }
            }
            const posible = dfs(to, operation)
            if (!posible) return false

        }

        return true
    }
    for (const node of uniqueNodes) {
        if (!dfs(node, true)) return false
        equalSet.clear()
        nonEqualSet.clear()

    }
    return true
};
```

## union find submission 

```javascript
/**
 * @param {string[]} equations
 * @return {boolean}
 */
var equationsPossible = function (equations) {
    const parents = {}
    const weights = {}
    const find = (node) => {
        let p = parents[node]
        while (p !== parents[p]) {
            p = parents[p] = parents[parents[p]]
        }
        return p
    }
    const union = (from, to) => {
        const parentFrom = find(from)
        const parentTo = find(to)
        if (parentFrom === parentTo) {
            return false
        }
        if (weights[parentFrom] > weights[parentTo]) {
            parents[parentTo] = parentFrom
            weights[parentFrom] += weights[parentTo]
        } else {
            parents[parentFrom] = parentTo
            weights[parentTo] += weights[parentFrom]
        }
        return true
    }
    const delayed = []
    for (const equation of equations) {
        const from = equation[0]
        const to = equation[3]
        const operation = equation[1] !== '!'
        if (!(from in parents)) {
            parents[from] = from
            weights[from] = 1
        }
        if (!(to in parents)) {
            parents[to] = to
            weights[to] = 1
        }
        if (operation) {
            union(from, to)
            continue
        }
        delayed.push([from, to])
    }
    for (const [from, to] of delayed) {
        if (from === to) return false
        if (!(to in parents) || !(from in parents)) continue
        const pFrom = find(from)
        const pTo = find(to)
        if (pFrom === pTo) return false
    }
    return true
};
```