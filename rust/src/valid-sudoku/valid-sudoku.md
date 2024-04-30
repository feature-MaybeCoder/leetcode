
#medium 
#matrix
#leetcode 
#array

[[valid-sudoku.excalidraw]]
last submission:
```javascript
/**

* @param {character[][]} board

* @return {boolean}

*/

var isValidSudoku = function(board) {

	const columnMap = Array(9).fill(0).map(()=>({}))
	
	let cellMap = []

	for(let i = 0; i < board.length; i ++){
	
		if(!(i%3)){
		
			cellMap = Array(3).fill(0).map(()=>({}))
		
		}

		let rowMap = {}
	
		for(let j = 0; j < board[i].length; j++){
	
			const value = board[i][j]
			
			if(value===".")continue
			
			if(value in rowMap)return false
			
			if(value in columnMap[j])return false
			
			let cellIndex = Math.floor(j/3)
			
			if(value in cellMap[cellIndex])return false
	
	  
	
			rowMap[value] = true
			
			columnMap[j][value] = true
			
			cellMap[cellIndex][value] = true
		}
	}
	
	return true

};
```
solution explanation:  validate each value in row keep in memory only current row, calculate cell index based on curent row value index, keep in memory only three cell, clear cell array every third row, validate each column, keep columns in memory to the end of program