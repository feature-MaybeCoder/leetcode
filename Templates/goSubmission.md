
<%*
const title = await tp.system.prompt("Set Folder name")
let argsAmount = await tp.system.prompt("Set func args amount")
if (!argsAmount || Number.isNaN(argsAmount)){
	argsAmount = 1
}else{
	argsAmount= Number(argsAmount)
}
const excalidrawName = title + ".excalidraw"
const goFileName = title + ".go"
const goFileTestName = title+"_test" + ".go"
const folderPath = "go/src/" + title;
const titleRelative = "go/" + "src/" + title;



await this.app.vault.createFolder(titleRelative)
const folder = await app.vault.getAbstractFileByPath(titleRelative)
await tp.file.rename(title)
await tp.file.move(titleRelative + "/" + title)
tp.file.create_new(tp.file.find_tfile("leetcodeDraw.excalidraw"), excalidrawName, false, folder)
function genArgs(amount){
	let args = ""
	for (let i =0; i <amount; i++){
		args += `arg${i+1} int${i === amount-1?'':', '}`
	}
	return args
}
function genArgsTestForFormat(amount){
	let args = ""
	for (let i =0; i <amount; i++){
		args += `tt.arg${i+1}${i === amount-1?'':', '}`
	}
	return args
}
function genArgsTestFormat(amount){
	let args = ""
	for (let i =0; i <amount; i++){
		args += `%v${i === amount-1?'':', '}`
	}
	return args
}
function genTestTypes(amount){
	let args = ""
	for (let i =0; i <amount; i++){
		args += `arg${i+1} int
		`
	}
	return args
}
function genTestArgs(amount){
	let args = ""
	for (let i =0; i <amount; i++){
		args += ` 0,`
	}
	return args
}
function goFileTemplate(name, argsAmount){

return `package ${name}
func ${name}(${genArgs(argsAmount)}){

}

`
}
function goFileTestTemplate(name, argsAmount){
return `package ${name}

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        ${genTestTypes(argsAmount)}
        want int
    }{
        {"base case",${genTestArgs(argsAmount)} 0},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := ${name}(${genArgsTestForFormat(argsAmount)})
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("${name}(${genArgsTestFormat(argsAmount)}) = %v, want %v", ${genArgsTestForFormat(argsAmount)}, got, tt.want)
            }
        })
    }
    }

`
}
await this.app.vault.create(titleRelative  + "/"+ goFileName, goFileTemplate(title, argsAmount))
await this.app.vault.create(titleRelative  + "/"+ goFileTestName, goFileTestTemplate(title,argsAmount))

const allTags = Object.entries(app.metadataCache.getTags() )
   .sort( (a, b) => a[0].localeCompare(b[0]) ) // Sorted alphabetically
   // .sort( (a, b) => b[1] - a[1], "desc" ) // Sorted related to frequency

let selectMore = true
let selectedTags = []
while (selectMore) {
  let choice = await tp.system.suggester((t) => t[0] + "(" + t[1] + ")", allTags, false, "[Select more tags (ESC when finished)] - " + selectedTags.join(", "))
  if (!choice) {
    selectMore = false
  } else {
    selectedTags.push(choice[0])
  }
}

tR += `
----

Tags: ${selectedTags.join(" ")}

----

## Drawing:
[[${title + ".excalidraw"}]]

----
`
%>

# Intuition

<!-- Describe your first thoughts on how to solve this problem. -->

  

# Approach

<!-- Describe your approach to solving the problem. -->

  

# Complexity

- Time complexity:

 $$O(n)$$

  

- Space complexity:

$$O(n)$$