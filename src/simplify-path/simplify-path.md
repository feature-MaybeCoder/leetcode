

----

Tags: #leetcode #medium #two-pointers

----

## Drawing:
[[simplify-path.excalidraw]]

----


## solution explanation:
simple solution with js methods: split string by '/' symbol, push if symbol anything but not .. or /, if symbol is .. pop last element from stack, join stack by '/' el plus one on the end.
more memory efitient, dont split string, iterate over it, keep stack in string format, return string

## last submission:
```javascript
/**
 * @param {string} path
 * @return {string}
 */
var simplifyPath = function (path) {

    const pathStack = []
    const pthes = path.split('/')

    for (let pathEl of pthes) {
        if (!pathEl) {
           continue
        }

        if (pathEl === '.') continue
        if (pathEl === '..') {
            if (pathStack.length) {
                pathStack.pop()
            }
            continue
        }
        
        pathStack.push(pathEl)

    }
    

    return '/' + pathStack.join('/')
};
```



