export function exit(code: number) {
    Deno.exit(code)
}

export async function execWasm(file: string): Promise<number> {
    return (await Deno.run({ cmd: ["deno", "run", "-A", "main.ts", file] }).status()).code
}