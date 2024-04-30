

----

Tags: #array #leetcode #medium #heap #quick-select

----

## Drawing:
[[kth-largest-element-in-an-array.excalidraw]]

----


# solution explanation:
## quick select solution: 
partition stage:
pick random piwot, move to right of array for simplicity
iterate over array if cur num is lg than piwot num, move number to left and increment left size counter, repeat process 
swap right (piwot el) to middle (left +1 )
when we return from partition stage we now how many elements in left and in right side that means that piwot element i left'ths smallest element and right'th largest

based on what we need move pointers to left or right
for example we need smallest element and cur piwot element position is larger
that means that target element that we need in the left from piwot element
## time complexity
average is On worst case is On^2

## Min heap solution
create min heap and pop K times

## Time complexity
n + K logN
n for add elements to heap, logn is complexity of popping, we need to pop k times

## last submission:
```javascript

```



