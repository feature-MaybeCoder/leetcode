

----

Tags: #leetcode #medium #binary-tree #repeat 

----

## Drawing:
[[construct-binary-tree-from-inorder-and-postorder-traversal.excalidraw]]

----


## solution explanation:
#tip: if we want to borrow pointer through recursions levels, just mutate an array (works only with most left pointer)
head of cur subtree is always the most left postortder element, pop it, find this el in inorder array

construct right from index+1, end of inorder, 
construct left from start of inorder, index-1


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
 * @param {number[]} inorder
 * @param {number[]} postorder
 * @return {TreeNode}
 */
var buildTree = function (inorder, postorder) {
    const constuct = (
        iStart, iEnd
    ) => {
        if ((iEnd - iStart) < 0) return null
        const head = postorder.pop()
        const iIndex = inorder.indexOf(head)

        const node = new TreeNode(head)

        const right = constuct(iIndex + 1, iEnd)
        const left = constuct(iStart, iIndex - 1)
        node.right = right
        node.left = left

        return node
    }
    return constuct(0, inorder.length - 1)
};
```
