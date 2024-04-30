#leetcode 
#dp 
#array 
#greedy
#recursion
#backtrack
[[jump-game.excalidraw]]

## solution explanation:
backtrack solution, start at end of the array, calculate new target if cur index val can reach prev target (init with last element), repeat the process, if last calculated target is reachable from cur index and cur index 0 than we can reach the end from start index

## last submission:
```javascript
/**

* @param {number[]} nums

* @return {boolean}

*/

var canJump = function(nums) {

	if(nums.length===1)return true
	
	let curTargetIndex = nums.length-1
	
	  
	
	for(let i =nums.length-2; i >=0; i--){
	
		const curStep = nums[i]
		
		if(i + curStep >= curTargetIndex){
			if(i===0)return true
			
			curTargetIndex=i
			
			continue
		
		}
		
		  
	
	}
	
	return false

};
```