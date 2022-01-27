function distanceNearestVowel(s: String): number {
  let distance: number = 0;
  let vowels: String[] = ['a', 'e', 'i', 'o', 'u'];
  for (let c of s) {
    let indexOfChar: number = vowels.indexOf(c)
    if (indexOfChar !== -1) {
      distance = indexOfChar;
      break;
    } else {
      continue;
    }
  }
  return distance
}

console.log(distanceNearestVowel("hello")); // should return 1