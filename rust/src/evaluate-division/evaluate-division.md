

----

Tags: #graph #medium #breath-first-search #depth-first-search #dp #leetcode #repeat 

----

## Drawing:
[[evaluate-division.excalidraw]]

----


## solution explanation:
traverse equations to graph and run bfs

## last submission:
```javascript
/**
 * @param {string[][]} equations
 * @param {number[]} values
 * @param {string[][]} queries
 * @return {number[]}
 */
var calcEquation = function (equations, values, queries) {
    const graph = {}

    for (let i = 0; i < equations.length; i++) {
        const curEquation = equations[i]
        const curEquationValue = values[i]
        if (!(curEquation[0] in graph)) {
            graph[curEquation[0]] = []
        }
        if (!(curEquation[1] in graph)) {
            graph[curEquation[1]] = []
        }
        graph[curEquation[0]].push([curEquation[1], curEquationValue])
        graph[curEquation[1]].push([curEquation[0], 1 / curEquationValue])

    }
    const results = []
    const bfs = (src, target) => {
        const
            visitedSet = new Set([src]),
            stack = [[src, 1]];
        let pointer = 0

        while (pointer < stack.length) {
            const [from, value] = stack[pointer]
            if (from === target) {
                return value
            }
            for (const connection of graph[from]) {
                if (visitedSet.has(connection[0])) {
                    continue
                }
                stack.push([connection[0], connection[1] * value])
                visitedSet.add(connection[0])
            }
            pointer++
        }
        return -1
    }
    for (let queryIndex = 0; queryIndex < queries.length; queryIndex++) {
        const qurQuery = queries[queryIndex]
        const devision = qurQuery[0]
        const devider = qurQuery[1]
        if (!(devision in graph) || !(devider in graph)) {
            results.push(-1)
            continue
        }
        results.push(bfs(devision, devider))

    }



    return results

};
```



