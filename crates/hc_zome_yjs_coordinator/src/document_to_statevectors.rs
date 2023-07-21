use hdk::prelude::*;
use hc_zome_yjs_integrity::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct AddStatevectorForDocumentInput {
    pub base_document_hash: ActionHash,
    pub target_statevector_hash: EntryHash,
}
#[hdk_extern]
pub fn add_statevector_for_document(input: AddStatevectorForDocumentInput) -> ExternResult<()> {
    create_link(input.base_document_hash.clone(), input.target_statevector_hash.clone(), LinkTypes::DocumentToStatevectors, ())?;
    

    Ok(())    
}

#[hdk_extern]
pub fn get_statevectors_for_document(document_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links = get_links(document_hash, LinkTypes::DocumentToStatevectors, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(EntryHash::from(link.target).into(), GetOptions::default()))
        .collect();

    // Get the records to filter out the deleted ones
    let records: Vec<Record> = HDK.with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();

    Ok(records)
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct GetStatevectorsForDocumentDelta {
    pub document_hash: ActionHash,
    pub statevectors: Vec<Statevector>
}
#[hdk_extern]
pub fn get_statevectors_for_document_delta(input: GetStatevectorsForDocumentDelta) -> ExternResult<Vec<Statevector>> {
    let all_statevectors = get_statevectors_for_document(input.document_hash)?;
    let all_statevectors_btreeset: BTreeSet<Statevector> = BTreeSet::from_iter(
        all_statevectors
            .iter()
            .filter_map(|r| r.entry()
                .to_app_option::<Statevector>()
                .ok()
            )
            .filter_map(|s| s)
    );

    let seen_statevectors_btreeset = BTreeSet::from_iter(input.statevectors.iter().cloned());
    let new_statevectors = all_statevectors_btreeset.difference(&seen_statevectors_btreeset).cloned().collect();

    Ok(new_statevectors)
}