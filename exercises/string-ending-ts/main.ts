function solution(str: String, ending: String): boolean {
  return str.substring(str.length - ending.length) == ending ? true : false;
}

console.log(solution("Hello", "llo"))