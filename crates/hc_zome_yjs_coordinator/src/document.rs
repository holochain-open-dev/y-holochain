use hdk::prelude::*;
use hc_zome_yjs_integrity::*;
#[hdk_extern]
pub fn create_document(document: Document) -> ExternResult<Record> {
    let document_hash = create_entry(&EntryTypes::Document(document.clone()))?;
    let record = get(document_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Document"))
            ),
        )?;
    let path = Path::from("all_documents");
    create_link(
        path.path_entry_hash()?,
        document_hash.clone(),
        LinkTypes::AllDocuments,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_document(original_document_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_document_hash.clone(),
        LinkTypes::DocumentUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_document_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_document_hash.clone(),
    };
    get(latest_document_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDocumentInput {
    pub original_document_hash: ActionHash,
    pub previous_document_hash: ActionHash,
    pub updated_document: Document,
}
#[hdk_extern]
pub fn update_document(input: UpdateDocumentInput) -> ExternResult<Record> {
    let updated_document_hash = update_entry(
        input.previous_document_hash.clone(),
        &input.updated_document,
    )?;
    create_link(
        input.original_document_hash.clone(),
        updated_document_hash.clone(),
        LinkTypes::DocumentUpdates,
        (),
    )?;
    let record = get(updated_document_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Document"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_document(original_document_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_document_hash)
}
