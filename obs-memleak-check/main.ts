import { globToRegExp, join } from "@std/path";
import { walk } from "@std/fs";

const toSearch = Deno.args[0] || prompt("Which directory do you want to check?")
if (!toSearch) {
  console.error("No directory provided.")
  Deno.exit(1)
}


const searchDir = join("..", toSearch)
const res = walk(searchDir, {
  match: [globToRegExp(join(searchDir, "src", "**", "*.rs"), {
    extended: true,
    globstar: true
  })]
})

const matches = [] as string[]
for await (const f of res) {
  if (!f.isFile)
    continue

  const content = await Deno.readTextFile(f.path)
  for (const res of content.matchAll(/obs_[A-z]*(?=\()/gm)) {
    matches.push(res[0])
  }
}

const createReleasePairs = [
  [/.*_create/g, /.*_(release|destroy)/g, true],
  [/obs_.*_encoder_create/g, /obs_encoder_release/g, ""],
]


let matchedPairs = [] as string[]
const errPairs = [] as string[]

// Check here if valid
for (const pair of createReleasePairs) {
  const [create, release] = pair
  const createArray = matches.filter(m => create.test(m))
  const releaseArray = matches.filter(m => release.test(m))

  for (const c of createArray) {
    const funcName = c.replace("create", "")
    const releaseFunc = releaseArray.find(e => e.includes(funcName))

    if (!releaseFunc) {
      if(!errPairs.includes(c))
        errPairs.push(c)
    } else {
      const index = errPairs.findIndex(s => c === s)
      if (index !== -1)
        errPairs.splice(index, 1)
    }
  }

  matchedPairs = matchedPairs.concat(createArray, releaseArray)
}

const leftOver = matches.filter(m => !matchedPairs.includes(m))
console.log("Leftover matches:\n", leftOver.join(", "))

if (errPairs.length === 0)
  console.log("No memory leaks found (hopefully).")
else {
  console.log("Found memory leaks:")
  for (const err of errPairs) {
    console.log(err)
  }
}