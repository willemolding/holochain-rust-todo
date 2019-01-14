// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');

const { Config, Container } = require('@holochain/holochain-nodejs')

const dnaPath = "dist/bundle.json"

// IIFE to keep config-only stuff out of test scope
const container = (() => {
  const agentAlice = Config.agent("alice")

  const dna = Config.dna(dnaPath)

  const instanceAlice = Config.instance(agentAlice, dna)

  const containerConfig = Config.container([instanceAlice])
  return new Container(containerConfig)
})()

// Initialize the Container
container.start()

const app = container.makeCaller('alice', dnaPath)

test('Can create a list', (t) => {
  const create_result = app.call("lists", "main", "create_list", {list: {name: "test list"}})
  console.log(create_result)
  t.notEqual(create_result.Ok, undefined)
  t.end()
})

test('Can add some items', (t) => {
  const create_result = app.call("lists", "main", "create_list", {list: {name: "test list"}})
  const list_addr = create_result.Ok

  const result1 = app.call("lists", "main", "add_item", {list_item: {text: "Learn Rust", completed: true}, list_addr: list_addr})
  const result2 = app.call("lists", "main", "add_item", {list_item: {text: "Master Holochain", completed: false}, list_addr: list_addr})

  console.log(result1)
  console.log(result2)

  t.notEqual(result1.Ok, undefined)
  t.notEqual(result2.Ok, undefined)

  t.end()
})

test('Can get a list with items', (t) => {
  const create_result = app.call("lists", "main", "create_list", {list: {name: "test list"}})
  const list_addr = create_result.Ok

  app.call("lists", "main", "add_item", {list_item: {text: "Learn Rust", completed: true}, list_addr: list_addr})
  app.call("lists", "main", "add_item", {list_item: {text: "Master Holochain", completed: false}, list_addr: list_addr})

  const get_result = app.call("lists", "main", "get_list", {list_addr: list_addr})
  console.log(get_result)

  t.equal(get_result.Ok.items.length, 2, "there should be 2 items in the list")
  t.end()
})
