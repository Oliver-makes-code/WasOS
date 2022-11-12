import { instantiate } from "./lib/wasos_sh.generated.js";

try {
    await Deno.mkdir("wasos")
} catch {/*Ignored!*/}

if (Deno.args[0]) {
    const mod = await instantiate({ url: new URL(Deno.args[0], import.meta.url)});

    await mod.main_fn()
} else {
    const mod = await instantiate();

    await mod.wasos_main()
}
