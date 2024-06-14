package leetcode

func sortColors(nums []int)  {
    left := 0
    mid := 0
    right := len(nums)-1
    for mid <= right{
        if nums[mid] == 0{
            nums[left], nums[mid] = nums[mid], nums[left]
            left++
            mid++
            continue
        }
        if nums[mid] == 1{
            mid++
            continue
        }
        nums[mid], nums[right] = nums[right], nums[mid]
        right--
    }
}