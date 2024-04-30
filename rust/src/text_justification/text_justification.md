

----

Tags: #leetcode #hard #string

----

## Drawing:
[[text_justification.excalidraw]]

----


## solution explanation:
split problem into two sub problems: 
	selection problem: amount of words in current line, solve by iterating over all words and add word len|len+1 (space) to sum of width, if width is less than max width continue to next word, else justify selected words
#### justification problem:
based on line index select middle or left justification
#### middle justification:
find min (floor) and max (ceil) possible space between words, select min or max with rules: if line spaces left amount could be filled with min amount of spaces, select min, else pick max amount (by this rule wi solve that If the number of spaces on a line does not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.)
#### left justification:
add for each one space, in the end of the loop add remaining spaces