

----

Tags: #leetcode #graph #medium #breath-first-search

----

## Drawing:
[[the_time_when_the_network_becomes_idle.excalidraw]]

----


## solution explanation:
basic bfs for shortest path calculation, for calculating delay time of current node we need: multiply cur depth by two (because we need to travel back and forth) if depth is divisible by patient, that means we need to sub patient from it that will lead us to delay of last sended message, else if depth is not devisible by patient, we need to sub from it remaining from div, add to that value depth by two, update max
