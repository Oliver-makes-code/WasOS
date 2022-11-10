const textEncoder = new TextEncoder()
export function std$write(str: string) {
    Deno.stdout.writeSync(textEncoder.encode(str))
}

export function std$writeln(str: string) {
    std$write(str+"\n")
}