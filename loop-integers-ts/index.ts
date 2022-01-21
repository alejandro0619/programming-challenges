import { createInterface } from 'readline' ;
// loop through number from 0 until num_limit
// Display those odds
function loop(num_limit: number) {
  for (let i = 0; i <= num_limit; i++) {
    if(i % 2 !== 0 && i > 0){
      console.log(i)
    } else {
      continue;
    }
  }
}
// Get input
function getInput(): void {
  const rl = createInterface({
    input: process.stdin,
    output: process.stdout
  });
  rl.question('This program will loop from 1 to the limit you enter, showing only those odds numbers \n', (num: string) =>{
    loop(parseInt(num));
    rl.close();
  });
}

getInput();