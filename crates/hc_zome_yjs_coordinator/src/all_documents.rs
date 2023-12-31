use hc_zome_yjs_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn get_all_documents(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_documents");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllDocuments, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .filter_map(|link| AnyDhtHash::try_from(link.target).ok())
        .map(|hash| GetInput::new(hash, GetOptions::default()))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().flatten().collect();
    Ok(records)
}
