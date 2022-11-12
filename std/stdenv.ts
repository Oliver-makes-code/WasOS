type Env = {
    [key:string]: string
}

async function getOrCreateEnv(): Promise<Env> {
    try {
        return JSON.parse(await Deno.readTextFile("env.json"))
    } catch(_) {
        Deno.create("env.json")
        return {}
    }
}

const env = await getOrCreateEnv()

export function getEnv(key: string): string {
    return env[key] ?? ""
}

export function setKey(key: string, val: string) {
    env[key] = val
}