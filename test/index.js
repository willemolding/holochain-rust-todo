// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require("tape");

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json");

// activate the new instance
app.start();

test("Can create a list", t => {
  const create_result = app.call(
    "lists",
    "main",
    "create_list",
    JSON.stringify({ list: { name: "test list" } })
  );
  t.equal(JSON.parse(create_result).success, true);
  t.end();
});

test("Can add some items", t => {
  const create_result = app.call(
    "lists",
    "main",
    "create_list",
    JSON.stringify({ list: { name: "test list" } })
  );
  const list_addr = JSON.parse(create_result).address;

  const result1 = app.call(
    "lists",
    "main",
    "add_item",
    JSON.stringify({
      list_item: { text: "Learn Rust", completed: true },
      list_addr: list_addr
    })
  );
  const result2 = app.call(
    "lists",
    "main",
    "add_item",
    JSON.stringify({
      list_item: { text: "Master Holochain", completed: false },
      list_addr: list_addr
    })
  );

  t.equal(JSON.parse(result1).success, true);
  t.equal(JSON.parse(result2).success, true);

  t.end();
});

test("Can get a list with items", t => {
  const create_result = app.call(
    "lists",
    "main",
    "create_list",
    JSON.stringify({ list: { name: "test list" } })
  );
  const list_addr = JSON.parse(create_result).address;

  app.call(
    "lists",
    "main",
    "add_item",
    JSON.stringify({
      list_item: { text: "Learn Rust", completed: true },
      list_addr: list_addr
    })
  );
  app.call(
    "lists",
    "main",
    "add_item",
    JSON.stringify({
      list_item: { text: "Master Holochain", completed: false },
      list_addr: list_addr
    })
  );

  const get_result = app.call(
    "lists",
    "main",
    "get_list",
    JSON.stringify({ list_addr: list_addr })
  );

  t.equal(JSON.parse(get_result).items.length, 2);
  t.end();
});

// for users list

test("Can create a list of users", t => {
  const create_result = app.call(
    "users",
    "main",
    "create_users_list",
    JSON.stringify({ list: { name: "test list" } })
  );
  t.equal(JSON.parse(create_result).success, true);
  t.end();
});

test("Can add some users to the list of users", t => {
  const create_result = app.call(
    "users",
    "main",
    "create_users_list",
    JSON.stringify({ list: { name: "test list" } })
  );
  const list_addr = JSON.parse(create_result).address;

  const result1 = app.call(
    "users",
    "main",
    "add_user",
    JSON.stringify({
      user: {
        name: "some name",
        email: "some email",
        phone: "some phone number"
      },
      list_addr: list_addr
    })
  );
  const result2 = app.call(
    "users",
    "main",
    "add_user",
    JSON.stringify({
      user: {
        name: "some other name",
        email: "some other email",
        phone: "some other phone number"
      },
      list_addr: list_addr
    })
  );

  t.equal(JSON.parse(result1).success, true);
  t.equal(JSON.parse(result2).success, true);

  t.end();
});

test("Can get a list of all users", t => {
  const create_result = app.call(
    "users",
    "main",
    "create_users_list",
    JSON.stringify({ list: { name: "test list" } })
  );
  const list_addr = JSON.parse(create_result).address;

  app.call(
    "users",
    "main",
    "add_user",
    JSON.stringify({
      user: {
        name: "some name",
        email: "some email",
        phone: "some phone number"
      },
      list_addr: list_addr
    })
  );

  app.call(
    "users",
    "main",
    "add_user",
    JSON.stringify({
      user: {
        name: "some other name",
        email: "some other email",
        phone: "some other phone number"
      },
      list_addr: list_addr
    })
  );

  const get_result = app.call(
    "users",
    "main",
    "get_users_list",
    JSON.stringify({ list_addr: list_addr })
  );

  t.equal(JSON.parse(get_result).items.length, 2);
  t.end();
});
