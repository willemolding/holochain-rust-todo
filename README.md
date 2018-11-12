# holochain-rust-todo

Basic example using holochain-rust

Build under the 0.0.1 dev preview release of holochain-rust

### Part 1

Recreate this code base by tutorial following the [accompanying article](https://hackmd.io/jwdkYitQQGCJX3THfxO-2A#)

### Part 2

( In Progress )
Lets take this one step further. Say you want to have an individual take ownership of a list of items, and you want to be able to set each item in the list as owned by that user, to enable multiple users with multiple lists of items, and be able to retrieve those lists.

Let's take a look at how we would do this.

First, we will need to reference the HDK USER_AGENT variable.
This variable is provided by the hdk and gives access to the unique agent hash.

Then, when the user is creating a list, we will link that list to the user via the USER_AGENT hash and, we will do the same for when a user creates a new item.
