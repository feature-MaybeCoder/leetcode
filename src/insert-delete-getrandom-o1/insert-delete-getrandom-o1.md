

----

Tags: #hashMap #leetcode #medium #design

----

## Drawing:
[[insert-delete-getrandom-o1.excalidraw]]

----


## solution explanation:
#tip: use two map for indexing by index and value, and use swap technique 

## last submission:
```javascript
var RandomizedSet = function () {
    this.set = {}
    this.map = {}
    this.head = 0
};

/** 
 * @param {number} val
 * @return {boolean}
 */
RandomizedSet.prototype.insert = function (val) {
    if (val in this.map) return false
    this.head++
    this.set[this.head] = val
    this.map[val] = this.head
    return true
};

/** 
 * @param {number} val
 * @return {boolean}
 */
RandomizedSet.prototype.remove = function (val) {
    if (!(val in this.map)) return false
    const valIndex = this.map[val]
    const lastValue = this.set[this.head]
    this.map[lastValue] = valIndex
    this.set[valIndex] = lastValue
    delete this.set[this.head]
    delete this.map[val]
    this.head--
    return true
};

/**
 * @return {number}
 */
RandomizedSet.prototype.getRandom = function () {
    
    const index = Math.floor(Math.random() * (this.head - 1 + 1)) + 1;
    return this.set[index]
};

/** 
 * Your RandomizedSet object will be instantiated and called as such:
 * var obj = new RandomizedSet()
 * var param_1 = obj.insert(val)
 * var param_2 = obj.remove(val)
 * var param_3 = obj.getRandom()
 */
```



