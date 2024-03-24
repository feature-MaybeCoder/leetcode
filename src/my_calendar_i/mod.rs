struct MyCalendar {
    books: Vec<[i32; 2]>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        return Self {
            books: Vec::with_capacity(1000),
        };
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.books.len()==0{
            self.books.push([start,end]);
            return true
        }
        let mut left = 0;
        let mut right = self.books.len() - 1;
        let mut mid = left + right >> 1;
        while left < right {
            mid = left + right >> 1;
            let mid_interval = self.books[mid];
            if start > mid_interval[0] {
                if start < mid_interval[1] {
                    return false;
                }
                left = mid + 1;
                continue;
            }
            if end > mid_interval[0] {
                return false;
            }
            right = mid.checked_sub(1).unwrap_or(0);
        }
        let final_interval = self.books[left];
        if start > final_interval[0] {
            if start < final_interval[1] {
                return false;
            }
            self.books.insert(left + 1, [start, end]);
            return true;
        }
        if end > final_interval[0] {
            return false;
        }
        self.books.insert(left, [start, end]);
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::MyCalendar;

    #[test]
    fn base_case() {
        let mut cal = MyCalendar::new();
        assert!(cal.book(1, 9));
        assert!(cal.book(0, 1));
        assert_eq!(cal.book(10, 20),true);
        assert_eq!(cal.book(20, 30),true);
        assert_eq!(cal.book(29, 40),false);
        assert_eq!(cal.book(9, 10),true);
    }
    #[test]
    fn neagtive_case() {
        let books =[[20,29],[13,22],[44,50],[1,7],[2,10],[14,20],[19,25],[36,42],[45,50],[47,50],[39,45],[44,50],[16,25],[45,50],[45,50],[12,20],[21,29],[11,20],[12,17],[34,40],[10,18],[38,44],[23,32],[38,44],[15,20],[27,33],[34,42],[44,50],[35,40],[24,31]];
        let assertions = [true,false,true,true,false,true,false,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false];

        let mut cal = MyCalendar::new();
        for (index, book) in books.iter().enumerate(){
            assert_eq!(cal.book(book[0], book[1]), assertions[index]);
        }
    }
}
