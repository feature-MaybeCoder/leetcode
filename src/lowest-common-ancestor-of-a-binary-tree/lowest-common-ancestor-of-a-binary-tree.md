

----

Tags: #binary-tree #leetcode #medium #breath-first-search

----

## Drawing:
[[lowest-common-ancestor-of-a-binary-tree.excalidraw]]

----


## solution explanation:
two solution: first run two dfs, get each path, iterating over two path if cur item in each path is not equal return prev item
second solution: run dfs, return node with simple boolean logic, if we have each node, than we in target root, return itself,
else return one of node that exist

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
 * @param {TreeNode} p
 * @param {TreeNode} q
 * @return {TreeNode}
 */
var lowestCommonAncestor = function (root, p, q) {

  const dfs = (node, targetp, targetq) => {
    if (!node) return node
    if (node === targetp || node === targetq) {
      return node
    }
    const left = dfs(node.left, targetp, targetq)
    const right = dfs(node.right, targetp, targetq)

    if (left && right) return node
    return left || right

  }

  return dfs(root, p, q)


};
```



