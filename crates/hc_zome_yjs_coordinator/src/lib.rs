pub mod document_to_statevectors;
pub use document_to_statevectors::CreateStatevectorForDocumentInput;
pub mod all_documents;
pub mod document;
pub mod document_to_agents;
pub mod statevector;
use hdk::prelude::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut fns = BTreeSet::new();
    fns.insert((zome_info()?.name, FunctionName("recv_remote_signal".into())));
    let cap_grant_entry: CapGrantEntry = CapGrantEntry::new(
        String::from("remote signals"), // A string by which to later query for saved grants.
        ().into(), // Unrestricted access means any external agent can call the extern
        GrantedFunctions::Listed(fns),
    );
    create_cap_grant(cap_grant_entry)?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: CreateStatevectorForDocumentInput) -> ExternResult<()> {
    emit_signal(signal)?;
    Ok(())
}
