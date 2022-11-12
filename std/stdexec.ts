import * as stdmain from "./std.ts"

export function exit(code: number) {
    Deno.exit(code)
}

export async function execWasm(file: string): Promise<number> {
    return (await Deno.run({ cmd: ["deno", "run", "-A", "main.ts", stdmain.getRealPath(file)] }).status()).code
}