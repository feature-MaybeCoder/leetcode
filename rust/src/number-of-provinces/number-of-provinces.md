

----

Tags: #graph #medium #depth-first-search #breath-first-search #dp #leetcode

----

## Drawing:
[[number-of-provinces.excalidraw]]

----


## solution explanation:
bfs on each possible root with visited map

## last submission:
```javascript
/**
 * @param {number[][]} isConnected
 * @return {number}
 */
var findCircleNum = function (isConnected) {
    let provincesAmount = isConnected.length;
    let len = isConnected.length;



    let visitedSet = new Set()

    for (let i = 0; i < len; i++) {
        if (visitedSet.has(i)) continue
        let stack = [i]
        let index = 0
        while (index < stack.length) {
            const rowIndex = stack[index]
            const row = isConnected[rowIndex]
            visitedSet.add(rowIndex)

            for (let colIndex = 0; colIndex < len; colIndex++) {
                if (colIndex === rowIndex) continue
                let connection = row[colIndex]
                if (connection === 1 && !visitedSet.has(colIndex)) {
                    provincesAmount--
                    visitedSet.add(colIndex)
                    stack.push(colIndex)

                }

            }

            index++

        }
    }


    return provincesAmount
};
```



