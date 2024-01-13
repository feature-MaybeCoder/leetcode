

----

Tags: #leetcode #medium #design #tree

----

## Drawing:
[[design-add-and-search-words-data-structure.excalidraw]]

----


## solution explanation:
create tree where keyes are symbols, and childs are next node with symbol, each node can be end of some word and have childs of another word, for example: bad bade
symbol d is the end of word bad, but will have child e of word dade.
search: iterate over symbols, if symbol is . expand dfs to all childs of the current node,
if current node dosent have childs and we not in the end of the word, then this brunch of dfs is falsy

## last submission:
```javascript
var WordDictionary = function () {
    this.root = new Node()
};

/** 
 * @param {string} word
 * @return {void}
 */
WordDictionary.prototype.addWord = function (word) {
    let node = this.root
    const len = word.length - 1
    for (let i = 0; i < word.length; i++) {
        const symbol = word[i]
        node = node.addChild(symbol, i === len)
    }
};

/** 
 * @param {string} word
 * @return {boolean}
 */
WordDictionary.prototype.search = function (word, index = 0, node = this.root) {
    if (index === word.length) {
        return node.isEnd
    }
    if (node.len === 0 && index !== word.length - 1) return false
    const symbol = word[index]
    if (symbol === '.') {
        for (const child of node.childs.values()) {
            if (this.search(word, index + 1, child)) return true
        }
        return false
    } else {
        const next = node.childs.get(symbol)
        if (!next) return false
        if (!this.search(word, index + 1, next)) return false
    }
    return true
};

/** 
 * Your WordDictionary object will be instantiated and called as such:
 * var obj = new WordDictionary()
 * obj.addWord(word)
 * var param_2 = obj.search(word)
 */
class Node {
    constructor(val, isEnd) {
        this.isEnd = isEnd
        this.len = 0
        this.val = val
        this.childs = new Map()
    }
    addChild(val, isEnd) {
        if (this.childs.has(val)) {
            const node = this.childs.get(val)
            if (!node.isEnd && isEnd) {
                node.isEnd = isEnd
            }
            return node
        }
        this.len++
        const node = new Node(val, isEnd)
        this.childs.set(val, node)
        return node
    }
}
```



