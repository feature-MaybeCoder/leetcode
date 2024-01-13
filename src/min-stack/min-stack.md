

----

Tags: #design #leetcode #medium #stack

----

## Drawing:
[[min-stack.excalidraw]]

----


## solution explanation:
create two stack, one main and second will be holding minimum values, if push value is LE that last in min stack push to min stack, else we never needed this value, because it will be poped before reach minimum

## last submission:
```javascript
var MinStack = function () {
    this.stack = []
    this.minStack = []
};

/** 
 * @param {number} val
 * @return {void}
 */
MinStack.prototype.push = function (val) {
    this.stack.push(val)

    if (!this.minStack.length || val <= this.minStack[this.minStack.length - 1]) {
        this.minStack.push(val)
    }


};

/**
 * @return {void}
 */
MinStack.prototype.pop = function () {
    const val = this.stack.pop()
    if (val === this.minStack[this.minStack.length - 1]) {
        this.minStack.pop()
    }
};

/**
 * @return {number}
 */
MinStack.prototype.top = function () {
    return this.stack[this.stack.length - 1]
};

/**
 * @return {number}
 */
MinStack.prototype.getMin = function () {
    return this.minStack[this.minStack.length - 1]
};

/** 
 * Your MinStack object will be instantiated and called as such:
 * var obj = new MinStack()
 * obj.push(val)
 * obj.pop()
 * var param_3 = obj.top()
 * var param_4 = obj.getMin()
 */
```



