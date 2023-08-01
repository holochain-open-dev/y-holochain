use hdi::prelude::*;

pub fn validate_create_link_document_to_agents(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::try_from(base_address)
        .map_err(|e| wasm_error!(e))?;
    let record = must_get_valid_record(action_hash)?;
    let _document: crate::Document = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // Check the target hash is an agentpubkey
    let _ = AgentPubKey::try_from(target_address).map_err(|e| wasm_error!(e))?;
 
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_document_to_agents(
    action: DeleteLink,
    original_action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the delete action author matchs the create action author
    if action.author != original_action.author {
        return Ok(ValidateCallbackResult::Invalid("Delete action author must match create action author".into()));
    }

    // Check the delete action author is the target address
    if action.author != AgentPubKey::try_from(target_address).map_err(|e| wasm_error!(e))? {
        return Ok(ValidateCallbackResult::Invalid("Delete action author match link target".into()));
    }

    Ok(ValidateCallbackResult::Valid)
}
