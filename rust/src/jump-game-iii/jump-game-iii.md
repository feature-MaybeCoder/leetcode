

----

Tags: #leetcode #recursion #array #array #depth-first-search #decision-tree #breath-first-search #medium 

----

## Drawing:
[[jump-game-iii.excalidraw]]

----


## solution explanation:
basic dfs with binary decision binary tree and caching. if we already visit current index so we in loop and should return false, else dive in next step of decision tree. complexity is On of decision nodes that can lead us to 0

## last submission:
```javascript
/**
 * @param {number[]} arr
 * @param {number} start
 * @return {boolean}
 */
var canReach = function(arr, start) {
    const visitedCache = {
    }
    const dfs = (index)=>{
        if(index in visitedCache)return false
        visitedCache[index]=true
        const val = arr[index]
        if(val===0)return true
        const sum = index + val
        

        let returnFromPrev = false
        if(sum < arr.length){
            returnFromPrev = dfs(sum)
        }
        if(returnFromPrev)return true
        
        const dif = index - val
        if(dif >=0){
            returnFromPrev = dfs(dif)
        }
        if(returnFromPrev)return true

        return false
    }
    return dfs(start)

};
```


