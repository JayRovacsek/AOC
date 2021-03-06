import * as wasm from "./pkg/aoc_2020.js";
dataSubmit.addEventListener("click", () => {
  const vals = [...document.getElementById("formInput").elements]
    .filter((x) => x.value !== "")
    .map((x) => x.value);
  const result = wasm.solve(vals[0], vals[1]);
  document.getElementById("formOutput").innerHTML = result;
});
