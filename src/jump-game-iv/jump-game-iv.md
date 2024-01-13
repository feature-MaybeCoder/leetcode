

----

Tags: #hashMap #breath-first-search #array #dp #hard

----

## Drawing:
[[jump-game-iv.excalidraw]]

----


## solution explanation:


submission with recursion:
overflow stack with large inputs
```javascript
/**
 * @param {number[]} arr
 * @return {number}
 */
var minJumps = function(arr) {
    if(arr.length ===1)return 0
    const teleportMap = {

    }
    if(Object.keys(teleportMap).length === arr.length)return teleportMap.length-1

    const visitedCache = new Array(arr.length).fill(-1);

    for(let i=0; i< arr.length; i++){
        const val = arr[i]
        if(val in teleportMap){
            teleportMap[val].push(i)
            continue
        }
        teleportMap[val] = [i]
    }
    const bfs = (index, prevIndex = 0) => {
        
        if(index === arr.length-1)return 1
        let minAmountOfSteps = Infinity
        const teleportArr = teleportMap[arr[index]]

        if(visitedCache[index] !== -1)return visitedCache[index] +1

        if(index + 1 !== prevIndex){
            const foundedValue = bfs(index+1, index)
            if(foundedValue < minAmountOfSteps){
                minAmountOfSteps = foundedValue
            }
        }
        
        if(index - 1 !== prevIndex && index -1 > 0){
            const foundedValue = bfs(index-1, index)
            if(foundedValue < minAmountOfSteps){
                minAmountOfSteps = foundedValue
            }
        }
        if(visitedCache[index-1]!==-1){
            if(visitedCache[index-1] < minAmountOfSteps){
                minAmountOfSteps = visitedCache[index-1] + 1
            }
        }

        if(teleportArr.length > 1){
            teleportMap[arr[index]] = []

            for(let telIndex of teleportArr){
                if(telIndex === index)continue
                if(visitedCache[telIndex] !== -1){
                    if(visitedCache[telIndex] < minAmountOfSteps){
                        minAmountOfSteps = visitedCache[telIndex] +1
                        visitedCache[index] = minAmountOfSteps
                    }
                    continue
                }

                const foundedValue = bfs(telIndex, index)

                if(foundedValue < minAmountOfSteps){
                    minAmountOfSteps = foundedValue
                    visitedCache[index] = minAmountOfSteps
                }
            }
        }
        visitedCache[index] = minAmountOfSteps
        return minAmountOfSteps +1
    }

    return bfs(0) -1

};
```



