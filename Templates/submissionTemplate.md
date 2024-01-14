
<%*
const title = await tp.system.prompt("Set Folder name")
const excalidrawTitle = title + ".excalidraw"

const titleRelative = "src/" + title

await this.app.vault.createFolder(titleRelative)

const folder = await app.vault.getAbstractFileByPath(titleRelative)
await tp.file.rename(title)
await tp.file.move(titleRelative + "/" + title)

tp.file.create_new(tp.file.find_tfile("leetcodeDraw.excalidraw"), excalidrawTitle, false, folder)

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

