
<%*
const title = await tp.system.prompt("Set Folder name")
const excalidrawName = title + ".excalidraw"
const rustFileName = "mod.rs"
const titleRelative = "rust/" + "src/" + title;



await this.app.vault.createFolder(titleRelative)
const folder = await app.vault.getAbstractFileByPath(titleRelative)
await tp.file.rename(title)
await tp.file.move(titleRelative + "/" + title)
tp.file.create_new(tp.file.find_tfile("leetcodeDraw.excalidraw"), excalidrawName, false, folder)

function rustFileTemplate(name){
return `
pub fn ${name}(){

}
#[cfg(test)]
mod test {
    use super::${name};

    #[test]
    fn base_case() {
        assert_eq!(${name}(), 0);
    }
}
`
}
await this.app.vault.create(titleRelative  + "/"+ rustFileName, rustFileTemplate(title))

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