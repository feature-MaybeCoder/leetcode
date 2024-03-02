

----

Tags: #graph #dijkstra #medium #leetcode #dp #breath-first-search #astar #belman-ford

----

## Drawing:
[[cheapest-flights-within-k-stops.excalidraw]]

----


## solution explanation:
belman-for is bfs kind but we iterating over all connections 
this problem also could be completed using bfs and visited map with previous calculated prices

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

}
/**
 * @param {number} n
 * @param {number[][]} flights
 * @param {number} src
 * @param {number} dst
 * @param {number} k
 * @return {number}
 */
var findCheapestPrice = function (n, flights, src, dst, k) {
    const graph = {}

    for (const [from, to, price] of flights) {
        if (!graph[from]) {
            graph[from] = []
        }
        graph[from].push([to, price])
    }
    let minPrice = Infinity
    const maxDepth = k + 1
    const stack = new _Queue()
    stack.enqueue([src, 0, 0])
    const visitedMap = new Map()
    while (stack.head < stack.tail) {

        const [node, price, depth] = stack.dequeue()
        if (visitedMap.has(node) && visitedMap.get(node) < price) {
            continue
        }
        visitedMap.set(node, price)
        if (depth > maxDepth) {

            continue
        }
        if (node === dst) {
            if (price < minPrice) {
                minPrice = price
            }

            continue
        }
        if (!graph[node]) {

            continue
        }
        const newDepth = depth + 1
        if (newDepth > maxDepth) {

            continue
        }
        for (const [to, toPrice] of graph[node]) {

            stack.enqueue([to, toPrice + price, newDepth])
        }

    }
    if (minPrice === Infinity) return -1
    return minPrice
};
```
