package leetcode

import "strings"
var digits map[int]string = map[int]string{
    1: "One",
    2: "Two",
    3: "Three",
    4: "Four",
    5: "Five",
    6: "Six",
    7: "Seven",
    8: "Eight",
    9: "Nine",
}
var tens map[int]string = map[int]string{
    2: "Twenty",
    3: "Thirty",
    4: "Forty",
    5: "Fifty",
    6: "Sixty",
    7: "Seventy",
    8: "Eighty",
    9: "Ninety",
}

var ten map[int]string = map[int]string{
    0: "Ten",
    1: "Eleven",
    2: "Twelve",
    3: "Thirteen",
    4: "Fourteen",
    5: "Fifteen",
    6: "Sixteen",
    7: "Seventeen",
    8: "Eighteen",
    9: "Nineteen",
}
func getStringAcordDigits(n int)[]string{
    res := make([]string, 0)
    h := n /100
    n %= 100
    t := n /10
    n %= 10
    if h > 0{
        res = append(res, digits[h], "Hundred")
    }
    if t > 0{
        if t ==1{
            res = append(res, ten[n])
        }else{
            res = append(res, tens[t])
        }
    }
    if n > 0 && t != 1{
        res = append(res, digits[n])
    }
    return res
}
func numberToWords(num int) string {
    if num == 0 {
        return "Zero"
    }
    res := make([]string, 0)
    billions := num / 1000000000
    num = num % 1000000000
    
    millions := num / 1000000
    num = num % 1000000
    
    thousands := num / 1000
    num = num % 1000

    if billions > 0{
        res = append(res, getStringAcordDigits(billions)...)
        res = append(res, "Billion")
    }
    if millions > 0{
        res = append(res, getStringAcordDigits(millions)...)
        res = append(res, "Million")
    }
    if thousands > 0{
        res = append(res, getStringAcordDigits(thousands)...)
        res = append(res, "Thousand")
    }
    res = append(res, getStringAcordDigits(num)...)

    return strings.Join(res, " ")
}