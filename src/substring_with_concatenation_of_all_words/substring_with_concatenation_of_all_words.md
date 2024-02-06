

----

Tags: #leetcode #sliding-window #hard #string #2d_cache #permutation

----

## Drawing:
[[substring_with_concatenation_of_all_words.excalidraw]]

----


## solution explanation:
count words in list to hashmap

for each possible start index of 0..word.length start sliding window algo,
copy count hashmap
if cur window len is equal to needed permutation length and we count down all needed words, then push start index, and left window pointer to the right, update state by adding remove word from window
if slice of input string end..end+word.length is not contains needed word then we stop cure window, reset state, move right pointer to right, update left to equal right
if slice contains needed word, but state is not contains it, (means we already reached all needed words of this type), that means we in possible valid window, but we need to move left to right so we reach new state position

else we just update state and continue, we in possible valid window

#tip run sliding window for all possible position is required in some cases
#tip remove keys from state so we can check is word needs in 0(1) time
