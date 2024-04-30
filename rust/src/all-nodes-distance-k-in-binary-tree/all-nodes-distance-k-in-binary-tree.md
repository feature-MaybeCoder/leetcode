

----

Tags: #binary-tree #medium #graph #breath-first-search #depth-first-search #leetcode

----

## Drawing:
[[all-nodes-distance-k-in-binary-tree.excalidraw]]

----


## solution explanation:
#tip run dfs for adding parents to binary tree and convert tree to graph

## last submission:
```javascript
/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {TreeNode} target
 * @param {number} k
 * @return {number[]}
 */
var distanceK = function (root, target, k) {
    const dfs = (node, p) => {
        if (!node) return
        node.p = p
        dfs(node.right, node)
        dfs(node.left, node)
    }
    dfs(root, null)
    let stack = [target]
    let depth = 0
    let returnNodes = []

    while (stack.length) {
        const newStack = []
        if (depth === k) {
            for (let node of stack) {
                if (!node || node.visited) continue
                returnNodes.push(node.val)
            }
            return returnNodes
        }
        for (let node of stack) {
            if (!node || node.visited) continue
            node.visited = true
            newStack.push(node.left, node.right, node.p)
        }
        depth++
        stack = newStack
    }

    return returnNodes
};
```



