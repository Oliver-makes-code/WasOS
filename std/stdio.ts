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
