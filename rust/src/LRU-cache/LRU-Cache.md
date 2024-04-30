#cache
#medium
#leetcode
#design
#stack
#queue
#linkedList
#hashMap

[[LRU-cache.excalidraw]]

кейс: при get операции на последний элемент должны переместить его на верх стака что является O(n)

last submission:
```javascript
class Node {

constructor(

value

) {

this.value = value;

this.next = null;

this.prev = null

}

unlink(){

if(this.next){

this.next.prev = this.prev

}

if(this.prev){

this.prev.next = this.next

}

}

}

  

class LinkedList {

constructor() {

this.head = null;

this.tail = null;

this.length = 0;

}

push(val) {

if (!this.tail) this.head = this.tail = new Node(val);

else{

const node = new Node(val);

node.prev = this.tail

this.tail = this.tail.next = node

}

return this.tail

}

moveToFirst(node){

if(node === this.tail)return

if(node === this.head){

this.head = this.head.next

}

node.unlink()

node.prev = this.tail

this.tail.next = node

this.tail = node

}

}

  

// demo

const list = new LinkedList();

/**

* @param {number} capacity

*/

var LRUCache = function(capacity) {

this.capacity = capacity

this.els = 0

this.keyValue = {

  

}

this.linkedList = new LinkedList()

};

  

/**

* @param {number} key

* @return {number}

*/

LRUCache.prototype.get = function(key) {

if(!(key in this.keyValue))return -1

this.linkedList.moveToFirst(this.keyValue[key].node)

return this.keyValue[key].value

};

  

/**

* @param {number} key

* @param {number} value

* @return {void}

*/

LRUCache.prototype.put = function(key, value) {

if(key in this.keyValue){

const obj = this.keyValue[key]

obj.value = value

this.linkedList.moveToFirst(obj.node)

  

return

}

if(this.els === this.capacity){

const head = this.linkedList.head

delete this.keyValue[head.value]

head.value = key

this.keyValue[key] = {

node: head,

value

}

this.linkedList.moveToFirst(head)

return

}

const pushed = this.linkedList.push(key)

this.keyValue[key] = {

node: pushed,

value

}

this.els += 1

};

  

/**

* Your LRUCache object will be instantiated and called as such:

* var obj = new LRUCache(capacity)

* var param_1 = obj.get(key)

* obj.put(key,value)

*/
```
