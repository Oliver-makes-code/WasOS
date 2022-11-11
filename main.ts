import { instantiate } from "./lib/wasos.generated.js";

if (Deno.args[0]) {
    const mod = await instantiate({ url: new URL(Deno.args[0], import.meta.url)});

    await mod.main_fn()
} else {
    const mod = await instantiate();

    await mod.wasos_main()
}
