function FirstFactorial(num: number) { 
  let acc: number = 1;
  while( num >= 1 ){
    acc *= num; 
    num -= 1;
  }
  // code goes here  
  return acc; 

}
 
console.log( FirstFactorial(5) )