use hdk::prelude::*;
use hc_zome_yjs_integrity::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct AddAgentForDocumentInput {
    pub base_document_hash: ActionHash,
    pub target_agent: AgentPubKey,
}
#[hdk_extern]
pub fn add_agent_for_document(input: AddAgentForDocumentInput) -> ExternResult<()> {
    create_link(input.base_document_hash.clone(), input.target_agent.clone(), LinkTypes::DocumentToAgents, ())?;

    Ok(())    
}

#[hdk_extern]
pub fn remove_agent_for_document(input: AddAgentForDocumentInput) -> ExternResult<()> {
    let links = get_links(input.base_document_hash, LinkTypes::DocumentToAgents, None)?;

    let agent_links: Vec<Link> = links
        .into_iter()
        .filter(|link| AgentPubKey::try_from(link.target.clone()).ok().unwrap() == input.target_agent)
        .collect();

    for link in agent_links {
        delete_link(link.create_link_hash)?;
    }

    Ok(())    
}


#[hdk_extern]
pub fn get_agents_for_document(document_hash: ActionHash) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(document_hash, LinkTypes::DocumentToAgents, None)?;
    
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .filter_map(|link| AgentPubKey::try_from(link.target).ok())
        .collect();


    Ok(agents)
}
