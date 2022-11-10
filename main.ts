import { instantiate } from "./lib/wasos.generated.js";

const { test } = await instantiate();

console.log(test());