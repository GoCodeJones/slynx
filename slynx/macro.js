function component1(param0 = 5){
  const out = {prop0:param0}
  console.log('Hello world')
  return out;
}
function main(){
  return component1()
}
main();