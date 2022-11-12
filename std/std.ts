export function getRealPath(path: string): string {
    const check: string[] = []
    const split = path.split("/")
    for (const i of split) {
        if (i == "..") {
            if (check.length <= 0) continue
            check.pop()
        } else if (i != "." && i != "") {
            check.push(i)
        }
    }
    return "/"+check.join("/")
}

export function pathExists(path: string): boolean {
    try {
        return Deno.statSync("./wasos/"+getRealPath(path)).isDirectory
    } catch {
        return false
    }
}

let currPath = ""

export function getCurrPath(): string {
    return getRealPath(currPath)
}

export function setCurrPath(val: string) {
    currPath = getRealPath(val)
}