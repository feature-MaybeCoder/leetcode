

----

Tags: #binary-tree #medium #leetcode

----

## Drawing:
[[flatten-binary-tree-to-linked-list.excalidraw]]

----


## solution explanation:
run dfs, firstly go to the left head, when recursion is rollout set node left to null, find right leaf, set left leaf right to right branch, cache cur node leaf and set value to right leaf
more effective solution: run dfs with this rules: return from recrustion rightHead||leftHead||node (leaf of cur subtree) set right to node.left leaf add right pointer to left leaf to node.right
## last submission:
```javascript
/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {void} Do not return anything, modify root in-place instead.
 */
var flatten = function (root) {
    let headMap = new Map()
    const findLeaf = (node) => {
        while (true) {
            if (headMap.has(node)) return headMap.get(node)
            if (node.right) {
                node = node.right
                continue
            }
            return node
        }
    }
    const dfs = (node) => {
        if (!node) return null

        let leftHead = dfs(node.left)
        node.left = null
        const rightHead = dfs(node.right)
        if (leftHead) {
            findLeaf(leftHead).right = rightHead
        } else {
            leftHead = rightHead
        }
        if (rightHead) {
            headMap.set(node, findLeaf(rightHead))
        }
        node.right = leftHead
        return node
    }



    return dfs(root)
};
```

```javascript
/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {void} Do not return anything, modify root in-place instead.
 */
var flatten = function (root) {
    const dfs = (node) => {
        if (!node) return null

        let leftHead = dfs(node.left)
        let tempLeft = node.left
        node.left = null
        const rightHead = dfs(node.right)
        let tempRight = node.right
        node.right = tempLeft
        if (leftHead) {
            leftHead.right = tempRight
        }else{
            node.right = tempRight
        }
        return rightHead || leftHead || node
    }


    dfs(root)
    return root
};
```