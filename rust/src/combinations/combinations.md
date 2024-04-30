

----

Tags: #leetcode #medium #array

----

## Drawing:
[[combinations.excalidraw]]

----


## solution explanation:
start at 1, with empty array, iterate over N while n + k - length is LE than N, push to combination array cur i run dfs to i+1 number, #tip we can not lookup branches if branch root + cur combination length is will be larger than K, that will not change O notation, but change N dependency of K to /2

### benchmarks:

myAlgo:  5: 20349
someAlgo: 5: 21700
myAlgo: 7: 116280
someAlgo: 7: 137980
myAlgo:  10: 352716
someAlgo: 10: 616666
## last submission:
```javascript
/**
 * @param {number} n
 * @param {number} k
 * @return {number[][]}
 */
var combine = function (n, k) {
    let combinations = []
    

    const dfs = (root, combination = []) => {
        
        if (combination.length === k) {
            combinations.push([...combination])
            return
        }


        let len = combination.length
        for (let i = root; ((i + k) - 1) - len <= n; i++) {
            combination.push(i)
            dfs(i + 1, combination)
            combination.pop()
        }
    }
    dfs(1)
    return combinations
};
```



