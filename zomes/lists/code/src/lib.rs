#![feature(try_from)]
use std::convert::TryFrom;

#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate serde_json;

use hdk::error::ZomeApiResult;

use hdk::holochain_core_types::{
    hash::HashString,
    error::HolochainError,
    entry::Entry,
    dna::zome::entry_types::Sharing,
    entry::entry_type::EntryType,
    json::JsonString,
    cas::content::Address
};



#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct List {
	name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
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
            },
            links: [
                to!(
                    "listItem",
                    tag: "items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
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
                outputs: |result: JsonString|,
                handler: handle_create_list
            }
            add_item: {
                inputs: |list_item: ListItem, list_addr: HashString|,
                outputs: |result: JsonString|,
                handler: handle_add_item
            }
            get_list: {
                inputs: |list_addr: HashString|,
                outputs: |result: JsonString|,
                handler: handle_get_list
            }
        }
    }
}

fn handle_create_list(list: List) -> JsonString {
    let list_entry = Entry::new(EntryType::App("list".into()), list);
	match hdk::commit_entry(&list_entry) {
		Ok(address) => json!({"success": true, "address": address}).into(),
		Err(hdk_err) => hdk_err.into()
	}
}

fn handle_add_item(list_item: ListItem, list_addr: HashString) -> JsonString {
    let list_item_entry = Entry::new(EntryType::App("listItem".into()), list_item);

	match hdk::commit_entry(&list_item_entry) // commit the list item
		.and_then(|item_addr| {
			hdk::link_entries(&list_addr, &item_addr, "items") // if successful, link to list
		})
	 {
		Ok(_) => {
			json!({"success": true}).into()
		},
		Err(hdk_err) => hdk_err.into()
	}
}

fn handle_get_list(list_addr: HashString) -> JsonString {

    // try and get the list entry and ensure it is the data type we expect
    let maybe_list = hdk::get_entry(list_addr.clone())
        .map(|entry| List::try_from(entry.unwrap().value()));

	match maybe_list {
		Ok(Ok(list)) => {

            // try and load the list items and convert them to the correct struct
			let maybe_list_items = get_links_and_load(&list_addr, "items").map(|results| {
                results.iter().map(|get_links_result| {
                    ListItem::try_from(get_links_result.entry.value().clone()).unwrap()
                }).collect::<Vec<ListItem>>()
            });

            // if this was successful for all list items then return them
            match maybe_list_items {
                Ok(list_items) => json!({"name": list.name, "items": list_items}).into(),
                Err(hdk_err) => hdk_err.into()
            }

		},
        _ => json!({"successs": false, "message": "No list at this address"}).into()
	}
}	



/*----------  Helper functions  ----------*/

pub struct GetLinksLoadElement {
    pub address: HashString,
    pub entry: Entry
}

pub type GetLinksLoadResult = Vec<GetLinksLoadElement>;

pub fn get_links_and_load<S: Into<String>>(
    base: &HashString, 
    tag: S
) -> ZomeApiResult<GetLinksLoadResult>  {
    hdk::get_links(base, tag)
        .map(|result| {
            result.addresses().iter()
                .map(|address| {
                    hdk::get_entry(address.to_owned())
                        .map(|entry: Option<Entry>| {
                            GetLinksLoadElement{
                                address: address.to_owned(),
                                entry: entry.unwrap()
                            }
                        })
                        .ok()
                })
                .filter_map(|elem| elem)
                .collect()
        })
}