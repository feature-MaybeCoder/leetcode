fn next(nums: &Vec<i32>, index: usize)->usize{
        return nums[index] as usize
    }
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = 0;
    let mut fast = 0;
    loop{
        slow = next(&nums, slow);
        fast = next(&nums, next(&nums, fast));
        if slow== fast{
            break
        }
    }
    slow = 0;
    while slow != fast{
            slow = next(&nums, slow);
            fast = next(&nums, fast);
        }
        return slow as i32
    }
