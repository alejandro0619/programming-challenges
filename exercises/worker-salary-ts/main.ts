import { readLines } from "https://deno.land/std/io/mod.ts";

// We assume you will enter only whole numbers.
// And only Int32 compatible string.
async function getInput(): Promise<[string, string]>{
  const { value } = await readLines(Deno.stdin).next();
  console.log('Please enter the hours you worked and the cost per hour');
  const input: string[] = value.split(' ');
  return [input[0], input[1]];
}

function convert(workedHours: string, costPerHour: string): [number, number] {
  return [parseInt(workedHours), parseInt(costPerHour)]
}

function getSalary(workedHours: number, costPerHour: number): number {
  return workedHours * costPerHour;
}
async function main() {
  const [workedHours, costPerHour] = await getInput();
  const [convertedWorkedHours, convertedCostPerHour] = convert(workedHours, costPerHour);
  const salary = getSalary(convertedWorkedHours, convertedCostPerHour);
  console.log(salary);
}

main();