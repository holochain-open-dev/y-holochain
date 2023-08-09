import { ActionHash, AgentPubKey } from "@holochain/client";

export interface Statevector {
  data: Uint8Array;
}

export interface Document {
  title: string;
}

export interface CreateStatevectorForDocumentSignal {
  provenance: AgentPubKey,
  document_hash: ActionHash,
  statevector: Statevector,
}
