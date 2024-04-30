

----

Tags: #leetcode #medium #array

----

## Drawing:
[[4sum.excalidraw]]

----


## solution explanation:
sort input array for two reasons: duplicates management and find rest two sum with two pointers

solve using recursion: start at index 0, for each possible index: 0..len-4 etc roll into next step of recursion, when we reach len of two in current result, run two pointers with left pointer set to cur index, and right pointer to arr length, calculate local needed sum (target - cur_cum), if sum of values corresponding to pointers is larger than needed sum, move right to left, else move left to right, if sum is match, push copy res array to final result array and move two pointers
when rollout from dfs recursion move index

#tip for each index move we apply rule: move pointer while next value is equal to current, that way we deal with duplicates
