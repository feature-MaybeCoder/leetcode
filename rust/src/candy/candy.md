

----

Tags: #hard #leetcode #array

----

## Drawing:
[[candy.excalidraw]]

----


## solution explanation:
greedy solution: 
in two iterations: from start to end and from end to start:
assigned cookie to cur index in weights array with rules:
	if neighbor has less rating then cur item, that means we reach local peak and we need to find max amount of cookies between two neighbors, set cur weight to max +1
we need to repeat process because we can deal with case wen we reach decrement sequence of elements, and one iteration is not enough for proper calculation
