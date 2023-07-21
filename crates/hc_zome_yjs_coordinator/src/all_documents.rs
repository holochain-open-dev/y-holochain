use hdk::prelude::*;
use hc_zome_yjs_integrity::*;
#[hdk_extern]
pub fn get_all_documents(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_documents");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllDocuments, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}
