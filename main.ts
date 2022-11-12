try {
    await Deno.mkdir("wasos")
} catch {/*Ignored!*/}

const { instantiate } = (await import(Deno.args[0] ? Deno.args[0]+".generated.js" : "./lib/wasos_sh.generated.js"))

const mod = await instantiate();

await mod.main_fn()
