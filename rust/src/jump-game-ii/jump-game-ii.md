

----

Tags: #array #greedy #dp #sliding-window #backtrack 

----

## Drawing:
[[jump-game-ii.excalidraw]]

----


## solution explanation:
### Backtrack dp solution:
first jump depends on all next jumps so we start from end -1 of array and calculate how many jumps from this point it needs, if from this point we can reach the end its 1 jump else it minimum amount jumps from all previous points
### Greedy solution
start from start of the array and caculate the sliding window that we can reach from this point,
next iterate this window and calculate next window, if from any point of this window we can reach the end of array return cur amount of jumps+1 else star iterating over next window
## last submission:
### Backtrack:
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var jump = function(nums) {
  if(nums.length ===1)return 0
  const amountOfJumps = {

  }
  for(let i = nums.length-2; i >= 0; i--){
    if(nums[i] === 0){
      amountOfJumps[i] = Infinity
      continue
    }
    if(nums[i] + i >= nums.length -1){
      amountOfJumps[i] = 1
      continue 
    }
    let minPrevJumps = amountOfJumps[i+1]
    for(let j = nums[i]; j > 0; j--){
      if(amountOfJumps[ i + j] < minPrevJumps){
        minPrevJumps = amountOfJumps[i + j]
      }
    }
    amountOfJumps[i] = minPrevJumps + 1
  }
  return amountOfJumps[0]
};
```
### Greedy:
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var jump = function(nums) {
    if(nums.length===1)return 0
    let endOfSelection = nums[0]
    let startOfSelection = 1
    let numsOfJump = 1
    while(endOfSelection < nums.length-1){
        let nextEndOfSelection = endOfSelection
        for(let i = startOfSelection; i <= endOfSelection; i++){
            let sum = i + nums[i]
            if( sum > nextEndOfSelection){
                if(sum > nums.length-1)return numsOfJump+1
                nextEndOfSelection = sum
            }
        }
        startOfSelection = endOfSelection + 1
        endOfSelection = nextEndOfSelection
        numsOfJump++
    }
    return numsOfJump
};
```


