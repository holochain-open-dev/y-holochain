import { ActionHash } from "@holochain/client";

export interface Statevector {
  data: Uint8Array;
}

export interface Document {
  title: string;
}

export interface CreateStatevectorForDocumentInput {
  document_hash: ActionHash,
  statevector: Statevector,
}
