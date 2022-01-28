import { readLines } from "https://deno.land/std/io/mod.ts";

async function get_input(): Promise<string> {
  const { value: input } = await readLines(Deno.stdin).next();
  return input;
}

function convert(wh: string, ch: string) : [number, number] {
  return [parseInt(wh), parseInt(ch)];
}
function get_salary(wh: number, ch: number): number{
  return wh * ch;
}

function main(){

}

main();
