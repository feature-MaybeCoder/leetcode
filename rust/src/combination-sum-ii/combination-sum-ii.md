

----

Tags: #array #backtrack #cache #dp #medium #recursion

----

## Drawing:
[[combination-sum-ii.excalidraw]]

----


## solution explanation:
binary decision tree, on each step check if value in disabled map, if it is, continue cur step,
else recursive binary decision, add value to disabled map and continue and add value to pick arr, and pass ref to disabled map

## last submission:

```javascript
/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum2 = function (candidates, target) {
    const results = []
    candidates.sort((a, b) => a - b)
    let maxIndex = candidates.length
    let isEqualAdded = false
    const foundCombination = (index, curSum, disabledCanditates, pickedCandidates) => {
        
        if (curSum === target) {
            results.push(pickedCandidates.slice())
            return
        }
        if (index === maxIndex || curSum > target) return
        const curValue = candidates[index]
        if (curValue > target) return
        if (curValue === target) {
            if (!isEqualAdded) {
                results.push([curValue])
                isEqualAdded = true
            }
            return
        }
        index += 1
        if (curValue in disabledCanditates) {
            foundCombination(index, curSum, disabledCanditates, pickedCandidates)
            return
        }
        
        pickedCandidates.push(curValue)
        foundCombination(index, curSum + curValue, disabledCanditates, pickedCandidates)
        pickedCandidates.pop()
        disabledCanditates[curValue] = true
        foundCombination(index, curSum, disabledCanditates, pickedCandidates)
        delete disabledCanditates[curValue]

        

    }
    foundCombination(0, 0, {}, [])
    return results
};
```

