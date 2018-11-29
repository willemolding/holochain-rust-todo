// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs')

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

test('Can create a list', (t) => {
  const create_result = app.call("lists", "main", "create_list", {list: {name: "test list"}})
  console.log(create_result)
  t.equal(create_result.success, true)
  t.end()
})

test('Can add some items', (t) => {
  const create_result = app.call("lists", "main", "create_list", {list: {name: "test list"}})
  const list_addr = create_result.address

  const result1 = app.call("lists", "main", "add_item", {list_item: {text: "Learn Rust", completed: true}, list_addr: list_addr})
  const result2 = app.call("lists", "main", "add_item", {list_item: {text: "Master Holochain", completed: false}, list_addr: list_addr})

  console.log(result1)
  console.log(result2)

  t.equal(result1.success, true)
  t.equal(result2.success, true)

  t.end()
})

test('Can get a list with items', (t) => {
  const create_result = app.call("lists", "main", "create_list", {list: {name: "test list"}})
  const list_addr = create_result.address

  app.call("lists", "main", "add_item", {list_item: {text: "Learn Rust", completed: true}, list_addr: list_addr})
  app.call("lists", "main", "add_item", {list_item: {text: "Master Holochain", completed: false}, list_addr: list_addr})

  const get_result = app.call("lists", "main", "get_list", {list_addr: list_addr})
  console.log(get_result)

  t.equal(get_result.items.length, 2, "there should be 2 items in the list")
  t.end()
})
