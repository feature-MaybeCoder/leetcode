

----

Tags: #leetcode #hard #segment-tree

----

## Drawing:
[[longest_increasing_subsequence_ii.excalidraw]]

----


## solution explanation:
main idea:
create dp cache for previous values, for each item in input array find max in range of dp cache from array[i]-1-k to attay[i]-1, then update dp[array[i]-1] to founded max + 1
that will lead us to On mc and On^2 tc,
we can optimize tc by using segment tree over dp cache,
that will lead us to On mc and Onlogn tc

