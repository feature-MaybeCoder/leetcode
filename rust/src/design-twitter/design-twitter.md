

----

Tags: #leetcode #medium #design #hashMap #array

----

## Drawing:
[[design-twitter.excalidraw]]

----


## solution explanation:
similar to sort-k-linked-lists, implement with two datastructure: global Map with posts and map of maps of user folows. main goal is to implement efficient getNewsFeed method with On where n is not all posts of all users, but just a flowed users

## last submission:
```javascript
var Twitter = function () {
    this.folowingMap = new Map()
    this.userPosts = new Map()
    this.incrimentalId = 0
};

/** 
 * @param {number} userId 
 * @param {number} tweetId
 * @return {void}
 */
Twitter.prototype.postTweet = function (userId, tweetId) {
    const post = {
        incrimentalId: this.incrimentalId,
        tweetId
    }
    this.incrimentalId++
    if (this.userPosts.has(userId)) {
        const posts = this.userPosts.get(userId)
        posts.push(post)
        return

    }
    this.userPosts.set(userId, [post])

};

/** 
 * @param {number} userId
 * @return {number[]}
 */
Twitter.prototype.getNewsFeed = function (userId) {
    const folowing = this.folowingMap.get(userId)
    const foloweeStack = folowing
        ? [...folowing?.values()]?.map(userId => this.userPosts.get(userId)?.map(postId => postId)) : [this.userPosts.get(userId)?.map(post => post)]
    const returnedPosts = []
    let i = 0,
        recentId = -Infinity,
        recentI = 0
    while (returnedPosts.length < 10 && foloweeStack.length) {

        const foloweePosts = foloweeStack[i]

        if (foloweePosts === undefined) {
            foloweeStack.splice(i, 1)
            i--
            continue
        }

        const lastId = foloweePosts.length - 1
        const post = foloweePosts[lastId]
        
        if (post.incrimentalId > recentId) {
            recentI = i
            recentId = post.incrimentalId
        }
        if (i === foloweeStack.length - 1) {
            recentId = -Infinity
            i = 0
            returnedPosts.push(foloweeStack[recentI].pop().tweetId)
            if (!foloweeStack[recentI].length) {
                foloweeStack.splice(recentI, 1)
            }
            continue
        }
        i++
    }
    return returnedPosts
};

/** 
 * @param {number} followerId 
 * @param {number} followeeId
 * @return {void}
 */
Twitter.prototype.follow = function (followerId, followeeId) {
    if (this.folowingMap.has(followerId)) {
        const folowing = this.folowingMap.get(followerId)
        folowing.set(followeeId, followeeId)
        return
    }
    this.folowingMap.set(followerId, new Map([[followeeId, followeeId], [followerId, followerId]]))
};

/** 
 * @param {number} followerId 
 * @param {number} followeeId
 * @return {void}
 */
Twitter.prototype.unfollow = function (followerId, followeeId) {
    if (!this.folowingMap.has(followerId)) return
    const folowing = this.folowingMap.get(followerId)
    folowing.delete(followeeId)
};

/** 
 * Your Twitter object will be instantiated and called as such:
 * var obj = new Twitter()
 * obj.postTweet(userId,tweetId)
 * var param_2 = obj.getNewsFeed(userId)
 * obj.follow(followerId,followeeId)
 * obj.unfollow(followerId,followeeId)
 */
```



