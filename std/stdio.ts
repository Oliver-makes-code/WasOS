const textEncoder = new TextEncoder()
export function stdout$write(str: string) {
    Deno.stdout.writeSync(textEncoder.encode(str))
}

export function stdout$writeln(str: string) {
    stdout$write(str+"\n")
}

export function stdin$promptln(): string {
    return prompt("") ?? ""
}

export function stdfs$listDir(path: string): File[] {
    const out = [] as File[]
    for (const a of Deno.readDirSync("."+path)) {
        out.push(new File(a.name, a.isDirectory))
    }
    out.sort((a, b) => a.name > b.name ? 1 : -1)
    return out
}

export class File {
    name: string
    isDir: boolean

    constructor(name: string, isDir: boolean) {
        this.name = name
        this.isDir = isDir
    }
}