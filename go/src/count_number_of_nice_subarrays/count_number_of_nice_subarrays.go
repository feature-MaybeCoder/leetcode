package leetcode
func isOdd(num int)int{
    if num % 2 == 1{
        return 1
    }
    return 0
}
func numberOfSubarrays(nums []int, k int) int {
    res := 0
    left := 0
    right := 0
    am := 0
    rightCount := 1
    nLen := len(nums)
    for right < nLen{

        if am == k{
            for right < nLen && isOdd(nums[right]) == 0{
                rightCount += 1
                right++
            }
        }
        if am < k{
            rightCount = 1
            am += isOdd(nums[right])
            right++
            continue
        }
        res += rightCount
        am -= isOdd(nums[left])
        left += 1
    }
    for left < right && am >= k{
        if am == k{
            res += rightCount
        }
        am -= isOdd(nums[left])
        left += 1
    }
    
    return res
}
