#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use hdk::holochain_dna::zome::entry_types::Sharing;
use hdk::holochain_core_types::hash::HashString;


#[derive(Serialize, Deserialize)]
struct List {
	name: String
}

#[derive(Serialize, Deserialize)]
struct ListItem {
	text: String,
	completed: bool
}
 
define_zome! {
    entries: [
        entry!(
            name: "list",
            description: "",
            sharing: Sharing::Public,
            native_type: List,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list: List, _ctx: hdk::ValidationData| {
                Ok(())
            }
        ),
        entry!(
            name: "listItem",
            description: "",
            sharing: Sharing::Public,
            native_type: ListItem,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list_item: ListItem, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    ]
 
    genesis: || {
        Ok(())
    }
 
	functions: {
        // "main" is the name of the capability
        // "Public" is the access setting of the capability
        main (Public) {
            create_list: {
                inputs: |list: List|,
                outputs: |result: serde_json::Value|,
                handler: handle_create_list
            }
            add_item: {
                inputs: |list_item: ListItem, list_addr: HashString|,
                outputs: |result: serde_json::Value|,
                handler: handle_add_item
            }
            get_list: {
                inputs: |list_addr: HashString|,
                outputs: |result: serde_json::Value|,
                handler: handle_get_list
            }
        }
    }
}

fn handle_create_list(list: List) -> serde_json::Value {
	match hdk::commit_entry("list", json!(list)) {
		Ok(address) => json!({"success": true, "address": address}),
		Err(hdk_err) => json!({"success": false, "error": hdk_err})
	}
}

fn handle_add_item(list_item: ListItem, list_addr: HashString) -> serde_json::Value {
	match hdk::commit_entry("listItem", json!(list_item)) // commit the list item
		.and_then(|item_addr| {
			hdk::link_entries(&list_addr, &item_addr, "items") // if successful, link to list
		})
	 {
		Ok(_) => {
			json!({"success": true})
		},
		Err(hdk_err) => json!({"success": false, "error": hdk_err})
	}
}

fn handle_get_list(list_addr: HashString) -> serde_json::Value {
	match hdk::get_entry::<List>(list_addr.clone()) { // try and get the list
		Ok(Some(list)) => {
			match hdk::get_links(&list_addr, "items") { // if successful, try to load the linked items
				Ok(result) => {
					let items: Vec<ListItem> = result.links.iter()
						.map(|item_addr| hdk::get_entry(item_addr.to_owned()) )
						.filter_map(|elem: Result<Option<ListItem>, _>| elem.unwrap())
						.collect(); // collect all the items in to a list

					json!({"name": list.name, "items": items})
				},
				Err(hdk_err) => hdk_err.to_json()
			}
		},
		Ok(None) => json!({"error": "no list found at address"}),
		Err(hdk_err) => hdk_err.to_json()
	}
}	
