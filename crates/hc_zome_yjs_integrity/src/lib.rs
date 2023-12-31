pub mod document_to_statevectors;
pub use document_to_statevectors::*;
pub mod document;
pub use document::*;
pub mod statevector;
pub use statevector::*;
pub mod document_to_agents;
pub use document_to_agents::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Statevector(Statevector),
    Document(Document),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    DocumentUpdates,
    AllDocuments,
    DocumentToStatevectors,
    DocumentToAgents,
}
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::Statevector(statevector) => {
                    validate_create_statevector(EntryCreationAction::Create(action), statevector)
                }
                EntryTypes::Document(document) => {
                    validate_create_document(EntryCreationAction::Create(action), document)
                }
            },
            OpEntry::UpdateEntry {
                app_entry, action, ..
            } => match app_entry {
                EntryTypes::Statevector(statevector) => {
                    validate_create_statevector(EntryCreationAction::Update(action), statevector)
                }
                EntryTypes::Document(document) => {
                    validate_create_document(EntryCreationAction::Update(action), document)
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterUpdate(update_entry) => match update_entry {
            OpUpdate::Entry {
                original_action,
                original_app_entry,
                app_entry,
                action,
            } => match (app_entry, original_app_entry) {
                (EntryTypes::Document(document), EntryTypes::Document(original_document)) => {
                    validate_update_document(action, document, original_action, original_document)
                }
                (
                    EntryTypes::Statevector(statevector),
                    EntryTypes::Statevector(original_statevector),
                ) => validate_update_statevector(
                    action,
                    statevector,
                    original_action,
                    original_statevector,
                ),
                _ => Ok(ValidateCallbackResult::Invalid(
                    "Original and updated entry types must be the same".to_string(),
                )),
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterDelete(delete_entry) => match delete_entry {
            OpDelete::Entry {
                original_action,
                original_app_entry,
                action,
            } => match original_app_entry {
                EntryTypes::Statevector(statevector) => {
                    validate_delete_statevector(action, original_action, statevector)
                }
                EntryTypes::Document(document) => {
                    validate_delete_document(action, original_action, document)
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => match link_type {
            LinkTypes::DocumentUpdates => {
                validate_create_link_document_updates(action, base_address, target_address, tag)
            }
            LinkTypes::AllDocuments => {
                validate_create_link_all_documents(action, base_address, target_address, tag)
            }
            LinkTypes::DocumentToStatevectors => validate_create_link_document_to_statevectors(
                action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::DocumentToAgents => {
                validate_create_link_document_to_agents(action, base_address, target_address, tag)
            }
        },
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => match link_type {
            LinkTypes::DocumentUpdates => validate_delete_link_document_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::AllDocuments => validate_delete_link_all_documents(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::DocumentToStatevectors => validate_delete_link_document_to_statevectors(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::DocumentToAgents => validate_delete_link_document_to_agents(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
        },
        FlatOp::StoreRecord(store_record) => match store_record {
            OpRecord::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::Statevector(statevector) => {
                    validate_create_statevector(EntryCreationAction::Create(action), statevector)
                }
                EntryTypes::Document(document) => {
                    validate_create_document(EntryCreationAction::Create(action), document)
                }
            },
            OpRecord::UpdateEntry {
                original_action_hash,
                app_entry,
                action,
                ..
            } => {
                let original_record = must_get_valid_record(original_action_hash)?;
                let original_action = original_record.action().clone();
                let original_action = match original_action {
                    Action::Create(create) => EntryCreationAction::Create(create),
                    Action::Update(update) => EntryCreationAction::Update(update),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Original action for an update must be a Create or Update action"
                                .to_string(),
                        ));
                    }
                };
                match app_entry {
                    EntryTypes::Statevector(statevector) => {
                        let result = validate_create_statevector(
                            EntryCreationAction::Update(action.clone()),
                            statevector.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_statevector: Option<Statevector> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_statevector = match original_statevector {
                                Some(statevector) => statevector,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_statevector(
                                action,
                                statevector,
                                original_action,
                                original_statevector,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::Document(document) => {
                        let result = validate_create_document(
                            EntryCreationAction::Update(action.clone()),
                            document.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_document: Option<Document> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_document = match original_document {
                                Some(document) => document,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_document(
                                action,
                                document,
                                original_action,
                                original_document,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                }
            }
            OpRecord::DeleteEntry {
                original_action_hash,
                action,
                ..
            } => {
                let original_record = must_get_valid_record(original_action_hash)?;
                let original_action = original_record.action().clone();
                let original_action = match original_action {
                    Action::Create(create) => EntryCreationAction::Create(create),
                    Action::Update(update) => EntryCreationAction::Update(update),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Original action for a delete must be a Create or Update action"
                                .to_string(),
                        ));
                    }
                };
                let app_entry_type = match original_action.entry_type() {
                    EntryType::App(app_entry_type) => app_entry_type,
                    _ => {
                        return Ok(ValidateCallbackResult::Valid);
                    }
                };
                let entry = match original_record.entry().as_option() {
                    Some(entry) => entry,
                    None => {
                        if original_action.entry_type().visibility().is_public() {
                            return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                        } else {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    }
                };
                let original_app_entry = match EntryTypes::deserialize_from_type(
                    app_entry_type.zome_index,
                    app_entry_type.entry_index,
                    entry,
                )? {
                    Some(app_entry) => app_entry,
                    None => {
                        return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                    }
                };
                match original_app_entry {
                    EntryTypes::Statevector(original_statevector) => {
                        validate_delete_statevector(action, original_action, original_statevector)
                    }
                    EntryTypes::Document(original_document) => {
                        validate_delete_document(action, original_action, original_document)
                    }
                }
            }
            OpRecord::CreateLink {
                base_address,
                target_address,
                tag,
                link_type,
                action,
            } => match link_type {
                LinkTypes::DocumentUpdates => {
                    validate_create_link_document_updates(action, base_address, target_address, tag)
                }
                LinkTypes::AllDocuments => {
                    validate_create_link_all_documents(action, base_address, target_address, tag)
                }
                LinkTypes::DocumentToStatevectors => validate_create_link_document_to_statevectors(
                    action,
                    base_address,
                    target_address,
                    tag,
                ),
                LinkTypes::DocumentToAgents => validate_create_link_document_to_agents(
                    action,
                    base_address,
                    target_address,
                    tag,
                ),
            },
            OpRecord::DeleteLink {
                original_action_hash,
                base_address,
                action,
            } => {
                let record = must_get_valid_record(original_action_hash)?;
                let create_link = match record.action() {
                    Action::CreateLink(create_link) => create_link.clone(),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "The action that a DeleteLink deletes must be a CreateLink".to_string(),
                        ));
                    }
                };
                let link_type =
                    match LinkTypes::from_type(create_link.zome_index, create_link.link_type)? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                match link_type {
                    LinkTypes::DocumentUpdates => validate_delete_link_document_updates(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::AllDocuments => validate_delete_link_all_documents(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::DocumentToStatevectors => {
                        validate_delete_link_document_to_statevectors(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::DocumentToAgents => validate_delete_link_document_to_agents(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                }
            }
            OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterAgentActivity(agent_activity) => match agent_activity {
            OpActivity::CreateAgent { agent, action } => {
                let previous_action = must_get_action(action.prev_action)?;
                match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
    }
}
