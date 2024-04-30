

----

Tags: #graph #dijkstra #leetcode #medium #breath-first-search #depth-first-search #repeat 

----

## Drawing:
[[shortest-path-with-alternating-colors.excalidraw]]

----


## solution explanation:
create two graph red and blue, start queue with two possible zeroes blue and red. create two loop set for blue and red value, because we often want to change color for accesing single path nodes

## last submission:
```javascript
class _Queue {
    constructor() {
        this.elements = {};
        this.head = 0;
        this.tail = 0;
    }
    enqueue(element) {
        this.elements[this.tail] = element;
        this.tail++;
    }
    dequeue() {
        const item = this.elements[this.head];
        delete this.elements[this.head];
        this.head++;
        return item;
    }
    peek() {
        return this.elements[this.head];
    }
    get length() {
        return this.tail - this.head;
    }
    get isEmpty() {
        return this.length === 0;
    }
}
/**
 * @param {number} n
 * @param {number[][]} redEdges
 * @param {number[][]} blueEdges
 * @return {number[]}
 */
var shortestAlternatingPaths = function (n, redEdges, blueEdges) {
    const redGraph = Array.from({ length: n }, () => [])
    const blueGraph = Array.from({ length: n }, () => [])
    const results = Array.from({ length: n }, () => -1)

    const redVisited = new Set()
    const blueVisited = new Set()

    for (let [from, to] of redEdges) {

        if (from === to && from === 0) continue
        redGraph[from].push(to)
    }
    for (let [from, to] of blueEdges) {
        if (from === to && from === 0) continue
        blueGraph[from].push(to)
    }
    const stack = new _Queue()
    stack.enqueue([0, 0, false, 0])
    stack.enqueue([0, 0, true, 0])

    while (stack.length) {
        const [node, from, isRed, depth] = stack.dequeue()
        const selectedGraph = isRed ? redGraph : blueGraph
        if (isRed) {
            redVisited.add(node)
        } else {
            blueVisited.add(node)

        }
        if (results[node] === -1 || depth < results[node]) {
            results[node] = depth
        }
        for (const neighboor of selectedGraph[node]) {
            const newRed = !isRed
            if (newRed) {
                if (redVisited.has(neighboor)) {

                    continue
                }
            } else {
                if (blueVisited.has(neighboor)) {
                    continue
                }
            }
            if (neighboor === from) continue
            stack.enqueue([neighboor, node, newRed, depth + 1])
        }
    }

    return results

};
```
