

----

Tags: #leetcode #medium #math

----

## Drawing:
[[factorial_trailing_zeroes.excalidraw]]

----


## solution explanation:
number of zeros depends on amount of 5 number in our factorial, 
The reason why simply dividing the number by 5 doesn't always yield the correct number of trailing zeros is that some numbers contribute more than one factor of 5. For example, 25 contributes two factors because 25 = 5 * 5. This means that every multiple of 25 actually contributes at least two factors of 5 to the factorial.
To correctly calculate the number of trailing zeros in `n!`, you must consider not just the multiples of 5, but also the multiples of 25, 125, 625, and so on. This is because these numbers contribute additional factors of 5.

so knowing that we need to simply devide n by five while n is ge than five and add remainder to our return res