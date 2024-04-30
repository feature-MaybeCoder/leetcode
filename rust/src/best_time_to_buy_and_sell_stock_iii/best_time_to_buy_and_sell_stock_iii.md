

----

Tags: #leetcode #hard #dp #2d_cache

----

## Drawing:
[[best_time_to_buy_and_sell_stock_iii.excalidraw]]

----


## solution explanation:
each value in array of prices depends of what max operation we could done in past and in a future, knowing that problem we can create prefix an postfix max cache, prefix cache will contain max operation from left to right, postfix from right to left, with that implimentation each index is depends on sum of prefix.cur_index + postfix.cur_index because they are self exluded
