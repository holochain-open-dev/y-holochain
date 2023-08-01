use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Statevector {
    pub data: SerializedBytes,
}

pub fn validate_create_statevector(
    _action: EntryCreationAction,
    _statevector: Statevector,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_statevector(
    _action: Update,
    _statevector: Statevector,
    _original_action: EntryCreationAction,
    _original_statevector: Statevector,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Statevectors cannot be updated",
    )))
}
pub fn validate_delete_statevector(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_statevector: Statevector,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Statevectors cannot be deleted",
    )))
}
