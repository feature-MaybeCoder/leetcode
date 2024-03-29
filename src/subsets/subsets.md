

----

Tags: #leetcode #medium #recursion #depth-first-search #decision-tree 

----

## Drawing:
[[subsets.excalidraw]]

----


## solution explanation:
basic dfs + cache problem, start recursion from index 0, on each step we can move froward to index+1..len variations, it is 2^n.len combinations, so on each step we init variation array that will contains all possible variations at this step, move to next level, after processing next level clone all possible combinations of next level and push current value to it, add this combination to current level combinations array, in the end of this level, clone combinations array to cache, so when we hit this level again we dont need to process it again, and append result array with all this combinations
