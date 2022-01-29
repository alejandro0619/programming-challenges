function solution(str: string, ending: string): boolean {
  return str.substring(str.length - ending.length) == ending ? true : false;
}

console.log(solution("Hello", "llo"))