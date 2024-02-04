

----

Tags: #leetcode #hard #graph #topological-sort

----

## Drawing:
[[sort_items_by_groups_respecting_dependencies.excalidraw]]

----


## solution explanation:
create helper data structure from input:
1) map groups containing elements, if element has -1 group, then add element to his unque group and push it
2) create groups graph, such that: iterate over before items of cur element, if before item has another group, that means group of cur element has directed connection to before item group
3) run two topological sorting: first is group tp sort, but when we reach the leaf we don't push group index to res, instead we run tp sort onto all groups elements
4) for each topological sorting we manage cycle, if some of sorting return that it contains cycle, that means result can't be found and we need to return an empty array
#tip we can run multiple sub traversal onto different adj list using some if statements and by swapping visited maps
