use hdk::prelude::*;
use hc_zome_yjs_integrity::*;
use crate::document_to_statevectors::*;

#[hdk_extern]
pub fn create_statevector(statevector: Statevector) -> ExternResult<Record> {
    let statevector_hash = create_entry(&EntryTypes::Statevector(statevector.clone()))?;
    let record = get(statevector_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Statevector"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_statevector(statevector_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(statevector_hash, GetOptions::default())
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateStatevectorForDocumentInput {
    pub document_hash: ActionHash,
    pub statevector: Statevector,
}
#[hdk_extern]
pub fn create_statevector_for_document(input: CreateStatevectorForDocumentInput) -> ExternResult<Record> {
    let sv = create_statevector(input.statevector)?;
    if let Some(entry_data) = sv.action().entry_data() {
        add_statevector_for_document(AddStatevectorForDocumentInput {
            base_document_hash: input.document_hash,
            target_statevector_hash: entry_data.0.clone()
        })?;
    }
    
    Ok(sv)
}
