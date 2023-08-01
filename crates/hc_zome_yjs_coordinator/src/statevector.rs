use hdk::prelude::*;
use hc_zome_yjs_integrity::*;

#[hdk_extern]
pub fn create_statevector(statevector: Statevector) -> ExternResult<Record> {
    let statevector_hash = create_entry(EntryTypes::Statevector(statevector.clone()))?;
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
