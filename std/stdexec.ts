import * as stdmain from "./std.ts"

type WexJson = {
    path: string
}

export function exit(code: number) {
    Deno.exit(code)
}

export async function execWex(file: string): Promise<number> {
    const wex: WexJson = JSON.parse(await Deno.readTextFile("wasos/"+stdmain.getRealPath(file)))
    return (await Deno.run({ cmd: ["deno", "run", "-A", "main.ts", wex.path] }).status()).code
}