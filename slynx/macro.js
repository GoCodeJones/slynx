function component1(A){
  const out = {prop0:A??5}
  console.log('Hello world')
  return out;
}
function main(){
  return component1()
}
main();