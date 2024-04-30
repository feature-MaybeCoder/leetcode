

----

Tags: #graph #hard #leetcode #tarjans 

----

## Drawing:
[[critical-connections-in-a-network.excalidraw]]

----


## solution explanation:
traverse graph using tarjans algo:
init discovery and min time array, start dfs at node zero:

init node disc and low time to cur counter value (0 for first node)

increase counter

start traversing all of node neighbors

if neighbor is parent, continue

if neighbor is already discovered, set cur node low to min of cur low and neighbor low value

else, move to neighbor node and repeat process

with rollouting from neighbor recursion set node low to min of cur low and neighbor low, 

if node disc time is less than neighbor low value, that means we found critical edge

## last submission:
```javascript

```



