#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use hdk::{holochain_core_types::hash::HashString, holochain_dna::zome::entry_types::Sharing};

#[derive(Serialize, Deserialize)]
struct UsersList {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct List {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    phone: String,
}

define_zome! {
    entries: [
    entry!(
        name: "usersList",
        description: "Stores a list of users",
        sharing: Sharing::Public,
        native_type: UsersList,
        validation_package: || hdk::ValidationPackageDefinition::Entry,
        validation: |usersList: UsersList, _ctx: hdk::ValidationData| {
            Ok(())
        }
    ),
       entry!(
           name: "user",
           description: "Stores info about a user",
           sharing: Sharing::Public,
           native_type: User,
           validation_package: || hdk::ValidationPackageDefinition::Entry,
           validation: |user: User, validation_data: hdk::ValidationData| {
               Ok(())
           }
       )
]
    genesis: || {
        Ok(())
    }

    functions: {
        main (Public) {
            create_users_list: {
                inputs: |list: UsersList|,
                outputs: |result: serde_json::Value|,
                handler: handle_create_users_list
            }
            add_user: {
                inputs: |user: User, list_addr: HashString|,
                outputs: |result: serde_json::Value|,
                handler: handle_add_user
            }
            get_users_list: {
                inputs: |list_addr: HashString|,
                outputs: |result: serde_json::Value|,
                handler: handle_get_users_list
            }
            // update user (on hold while core dev team implements update_entry)
        }
    }
}

fn handle_create_users_list(list: UsersList) -> serde_json::Value {
    match hdk::commit_entry("list", json!(list)) {
        Ok(address) => json!({"success": true, "address": address}),
        Err(hdk_err) => json!({"success": false, "error": hdk_err}),
    }
}

fn handle_add_user(user: User, list_addr: HashString) -> serde_json::Value {
    match hdk::commit_entry("user", json!(user)) // commit the list item
        .and_then(|item_addr| {
            hdk::link_entries(&list_addr, &item_addr, "users_item") // if successful, link to list
        }) {
        Ok(_) => json!({"success": true}),
        Err(hdk_err) => json!({"success": false, "error": hdk_err}),
    }
}

fn handle_get_users_list(list_addr: HashString) -> serde_json::Value {
    match hdk::get_entry::<List>(list_addr.clone()) {
        // try and get the list
        Ok(Some(list)) => {
            match hdk::get_links(&list_addr, "users_item") {
                // if successful, try to load the linked items
                Ok(result) => {
                    let users_items: Vec<User> = result
                        .links
                        .iter()
                        .map(|item_addr| hdk::get_entry(item_addr.to_owned()))
                        .filter_map(|elem: Result<Option<User>, _>| elem.unwrap())
                        .collect(); // collect all the items in to a list

                    json!({"name": list.name, "users_items": users_items })
                }
                Err(hdk_err) => hdk_err.to_json(),
            }
        }
        Ok(None) => json!({"error": "no list found at address"}),
        Err(hdk_err) => hdk_err.to_json(),
    }
}
