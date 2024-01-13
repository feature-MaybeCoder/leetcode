

----

Tags: #array #leetcode #medium

----

## Drawing:
[[h-index.excalidraw]]

----


## solution explanation:
sort citations iterate over it, if cur index GE than item return index else return len of arr

## last submission:
```javascript
/**
 * @param {number[]} citations
 * @return {number}
 */
var hIndex = function (citations) {
    citations.sort((a, b) => b - a)
    
    
    for (let i = 0; i < citations.length; i++) {
        const item = citations[i]
        if (i >= item) {
            return i
           
        }
    }
    return citations.length
};
```



