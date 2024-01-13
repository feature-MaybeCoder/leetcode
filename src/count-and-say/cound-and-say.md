#string 
#leetcode 
#medium 
#dp
[[count-and-say.excalidraw]]

## solution explanation:

start with based case initialize cause we now that n > 1
while iterating get prev pointer for switching when substring is end count element in repeating substring, when substring is end, push to sayStringTemp when we end the substring loop always push what we have because lat elements is always a substring, repeat process for n-1 times

## last submission:

```javascript
var countAndSay = function(n) {

	let sayString = "1"
	
	for(let i = 1; i < n; i++){
	
		let lesCount = 1
		
		let prev = sayString[0]
		
		let sayStringTemp = ""
		
		  
		
		for(let j = 1; j < sayString.length; j++){
		
			const cur = sayString[j]
			
			if(cur !== prev){
			
				sayStringTemp += lesCount + "" + prev
				
				prev = cur
				
				lesCount = 1
				
				continue
		
			}
		
			lesCount++
		
		}
		
		sayStringTemp += lesCount + "" + prev
		
		sayString = sayStringTemp
	
	}
	
	  
	
	return sayString

  

};
```