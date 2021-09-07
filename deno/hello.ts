console.log('hello deno')

const res = await fetch('https://pokeapi.co/api/v2/pokemon/1')
const json = res.json()
const data = await json
console.log(data)




