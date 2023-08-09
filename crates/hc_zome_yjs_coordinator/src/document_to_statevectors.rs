use crate::document_to_agents::*;
use crate::statevector::*;
use crate::utils::*;
use hc_zome_yjs_integrity::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct AddStatevectorForDocumentInput {
    pub base_document_hash: ActionHash,
    pub target_statevector_hash: EntryHash,
}
#[hdk_extern]
pub fn add_statevector_for_document(input: AddStatevectorForDocumentInput) -> ExternResult<()> {
    create_link_relaxed(
        input.base_document_hash,
        input.target_statevector_hash,
        LinkTypes::DocumentToStatevectors,
        (),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_statevectors_for_document(document_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links = get_links(document_hash, LinkTypes::DocumentToStatevectors, None)?;

    let get_input: Vec<GetInput> = links
        .into_iter()
        .filter_map(|link| AnyDhtHash::try_from(link.target).ok())
        .map(|hash| GetInput::new(hash, GetOptions::default()))
        .collect();

    // Get the records to filter out the deleted ones
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .flatten()
        .collect();

    Ok(records)
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct GetStatevectorsForDocumentDelta {
    pub document_hash: ActionHash,
    pub statevectors: Vec<Statevector>,
}
#[hdk_extern]
pub fn get_statevectors_for_document_delta(
    input: GetStatevectorsForDocumentDelta,
) -> ExternResult<Vec<Statevector>> {
    let all_statevectors = get_statevectors_for_document(input.document_hash)?;
    let all_statevectors_btreeset: BTreeSet<Statevector> = BTreeSet::from_iter(
        all_statevectors
            .iter()
            .filter_map(|r| r.entry().to_app_option::<Statevector>().ok())
            .flatten(),
    );

    let seen_statevectors_btreeset = BTreeSet::from_iter(input.statevectors.iter().cloned());
    let new_statevectors = all_statevectors_btreeset
        .difference(&seen_statevectors_btreeset)
        .cloned()
        .collect();

    Ok(new_statevectors)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateStatevectorForDocumentInput {
    pub document_hash: ActionHash,
    pub statevector: Statevector,
}
#[hdk_extern]
pub fn create_statevector_for_document(
    input: CreateStatevectorForDocumentInput,
) -> ExternResult<Record> {
    let sv = create_statevector(input.statevector)?;
    if let Some(entry_data) = sv.action().entry_data() {
        add_statevector_for_document(AddStatevectorForDocumentInput {
            base_document_hash: input.document_hash,
            target_statevector_hash: entry_data.0.clone(),
        })?;
    }

    Ok(sv)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateStatevectorForDocumentSignal {
    pub provenance: AgentPubKey,
    pub document_hash: ActionHash,
    pub statevector: Statevector,
}
#[hdk_extern]
pub fn remote_signal_statevector_for_document(
    input: CreateStatevectorForDocumentInput,
) -> ExternResult<()> {
    let mypubkey = agent_info()?.agent_initial_pubkey;
    let agents = get_other_agents_for_document(input.clone().document_hash)?;
    remote_signal(CreateStatevectorForDocumentSignal {
        provenance: mypubkey,
        document_hash: input.document_hash, 
        statevector: input.statevector
    }, agents)?;

    Ok(())
}

#[hdk_extern]
pub fn create_or_signal_statevector_for_document(
    input: CreateStatevectorForDocumentInput,
) -> ExternResult<()> {
    let mut links = get_links(
        input.document_hash.clone(),
        LinkTypes::DocumentToStatevectors,
        None,
    )?;
    links.sort_by_key(|l| l.timestamp);
    let maybe_newest_link = links.last();

    // If > 5 mins since last commit, publish commit
    if let Some(newest_link) = maybe_newest_link {
        if newest_link.timestamp.0 > sys_time()?.0 + (1000 * 60 * 5) {
            create_statevector_for_document(input.clone())?;
        }
    }

    // Always remote_signal_sv
    remote_signal_statevector_for_document(input)
}
