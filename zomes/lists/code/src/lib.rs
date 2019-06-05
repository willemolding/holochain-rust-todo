#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        hash::HashString,
        error::HolochainError,
        dna::entry_types::Sharing,
        json::JsonString,
        cas::content::Address,
        entry::Entry,
    }
};

 
define_zome! {
    entries: [
        entry!(
            name: "list",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<List>| {
                Ok(())
            },
            links: [
                to!(
                    "listItem",
                    link_type: "items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "listItem",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<ListItem>| {
                Ok(())
            }
        )
    ]
 
    genesis: || {
        Ok(())
    }
 
	functions: [
        create_list: {
            inputs: |list: List|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_list
        }
        add_item: {
            inputs: |list_item: ListItem, list_addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_item
        }
        get_list: {
            inputs: |list_addr: HashString|,
            outputs: |result: ZomeApiResult<GetListResponse>|,
            handler: handle_get_list
        }
    ]
    traits: {
        hc_public [create_list, add_item, get_list]
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct List {
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct ListItem {
    text: String,
    completed: bool
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct GetListResponse {
    name: String,
    items: Vec<ListItem>
}

fn handle_create_list(list: List) -> ZomeApiResult<Address> {
    // define the entry
    let list_entry = Entry::App(
        "list".into(),
        list.into()
    );

    // commit the entry and return the address
	hdk::commit_entry(&list_entry)
}


fn handle_add_item(list_item: ListItem, list_addr: HashString) -> ZomeApiResult<Address> {
    // define the entry
    let list_item_entry = Entry::App(
        "listItem".into(),
        list_item.into()
    );

	let item_addr = hdk::commit_entry(&list_item_entry)?; // commit the list item
	hdk::link_entries(&list_addr, &item_addr, "items", "")?; // if successful, link to list address
	Ok(item_addr)
}


fn handle_get_list(list_addr: HashString) -> ZomeApiResult<GetListResponse> {

    // load the list entry. Early return error if it cannot load or is wrong type
    let list = hdk::utils::get_as_type::<List>(list_addr.clone())?;

    // try and load the list items, filter out errors and collect in a vector
    let list_items = hdk::get_links(&list_addr, Some("items".to_string()), None)?.addresses()
        .iter()
        .map(|item_address| {
            hdk::utils::get_as_type::<ListItem>(item_address.to_owned())
        })
        .filter_map(Result::ok)
        .collect::<Vec<ListItem>>();

    // if this was successful then return the list items
    Ok(GetListResponse{
        name: list.name,
        items: list_items
    })
}
