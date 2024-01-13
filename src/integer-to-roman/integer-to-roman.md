

----

Tags: #leetcode #medium

----

## Drawing:
[[integer-to-roman.excalidraw]]

----


## solution explanation:
we have 4 main cases, when current symbol is LE than 3 when curent symbol is 4, when cur symbol is 9, or when cur cymbol is GE than 5, iterating over each symbol in number and add latin symbol acord number devision

## last submission:
```javascript
/**
 * @param {number} num
 * @return {string}
 */
var intToRoman = function (num) {
    const unit = {
        1: 'I',
        2: 'X',
        3: 'C',
        4: 'M'
    }
    const round = {
        1: 'V',
        2: 'L',
        3: 'D',
    }
    let string = num + ''
    let len = string.length
    let returnString = ''
    for (let i = 0; i < len; i++) {
        let curSymbol = string[i]
        let curNumber = Number(curSymbol)
        let devision = len - i
        
        if (curNumber <= 3) {
            for (let j = 0; j < curNumber; j++) {
                returnString += unit[devision]
            }
            continue
        }
        if (curNumber === 4) {
            returnString += unit[devision] + round[devision]
            continue
        }
        if (curNumber === 9) {
            returnString += unit[devision] + unit[devision + 1]
            continue
        }
        curNumber -= 5
        returnString += round[devision]
        while (curNumber>0) {
            returnString += unit[devision]
            curNumber--
        }
    }
    return returnString
};
```



