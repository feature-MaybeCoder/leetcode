

----

Tags: #leetcode #hard #heap

----

## Drawing:
[[sliding_window_median.excalidraw]]

----


## solution explanation:
### brutforce:
iterate over array, clone window and sort it, pick the median
tc: N * K(logK) (k is the size of a window)
### two heaps approach:
create left and right heap, left heap will be max heap and will contains all smaller or eq then median items (so we always track what the max of left portion of sorted array), right heap will be min heap and will contains all largest number to the right of the median, left heap will contains n+1 so to calculate a midan in odd case we need to simply peek left heap, in even case we need to peek left and right and calc average
#### initial balancing:
when we first populating initial window state we need to push item to the left, then pop from left and push to the right, so right will contains largest numbers of the window, if right len is greater then left, pop from right to left, so left will contain smallest number
#### runtime balancing:
in runtime we need to shift our window, remove last item and push new right item, first calculate current median, then remove prev number from our structure, push new number to structure
#### pushing rules:
if prev number is less then or equals to median, then somewhere in the future we will remove prev number from left heap, this means left heap is lack 1 element -> set balance to -1, else prev number is in right portion of sorted array and right heap contains it, so in the future we will remove it from the right, right heap is lack one number -> set balance 1,
similarly if next number is larger then median, we will push it to the right -> set balance -1 (left heap lack one item), else to the left -> set balance +1
balance heaps, if balance > 0 then left heap is contains 2+ more items, and we need to push to the right, push to the right popped from left. if balance is <0 -> left heap is lack an item, push to the left popped from right number
#### removing rules:
we need to remove only if item that we want to remove is in peek of current heap, so keep track of removed items in hashmap and pop from left and right while their peeks in removed hashmap