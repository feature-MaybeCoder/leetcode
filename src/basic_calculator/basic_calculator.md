

----

Tags: #leetcode #hard #stack

----

## Drawing:
[[basic_calculator.excalidraw]]

----


## solution explanation:
main problem is to combine recursion parentheses and reverse operators, create three stacks, main stack for integers, operator stack, and stack for parentheses recursion containing previous value of stack len and reverse len
#tip if we need recursion objects, we can rely on an helper stack that will contain the state as we were before moving to a new level of recursion, when we exit recursion, we simply pop this state from stack