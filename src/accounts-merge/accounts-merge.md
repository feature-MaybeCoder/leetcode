



----

Tags: #graph #leetcode #medium #union-find #hard #repeat #interesting 

----

## Drawing:
[[accounts-merge.excalidraw]]

----


## solution explanation:
init of data structure of accounts length, iterating over all emails, add email to Map<email,index>, if email already in map union current index and index that inside map by this email (that means we found new account with same email), iterate over all email in map (IMPORTANT: not over all emails, onli unique in map), reverte map from Map<email, index> to Map<index, email[]> iterate over new map entries, sort emails arr and concat with account name, that will be our return array

## last submission:
```javascript

```



