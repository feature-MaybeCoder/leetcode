
<%*
const title = await tp.system.prompt("Set Folder name")
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

function goFileTemplate(name){
return `package ${name}
func ${name}(){

}

`
}
function goFileTestTemplate(name){
return `package ${name}

import (
	"reflect"
	"testing"
)

func TestSolution(t *testing.T) {
    tests := []struct {
        name   string
        want int
    }{
        {"basic case", 0},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := ${name}()
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("${name}$() = %v, want %v", got, tt.want)
            }
        })
    }
    }

`
}
await this.app.vault.create(titleRelative  + "/"+ goFileName, goFileTemplate(title))
await this.app.vault.create(titleRelative  + "/"+ goFileTestName, goFileTestTemplate(title))

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

## solution explanation:
