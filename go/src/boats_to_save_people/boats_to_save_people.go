package boats_to_save_people
import (
    "sort"
)
func numRescueBoats(people []int, limit int) int {
    amount := 0
    sort.Ints(people)
    left := 0
    right := len(people)-1

    for left < right {
        if (people[left] + people[right] > limit){
            amount +=1
            right -=1
            continue
        }
        left+=1
        right -=1
        amount +=1
    }
    if left == right{
        amount +=1
    }
    return amount
}
