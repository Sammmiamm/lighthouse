use super::{
    ChainSpec, EthSpec, Fork, Hash256, SecretKey, Signature, SignedRoot, SyncCommitteeContribution,
    SyncSelectionProof,
};
use crate::test_utils::TestRandom;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use test_random_derive::TestRandom;
use tree_hash_derive::TreeHash;

/// A Validators aggregate sync committee contribution and selection proof.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TestRandom,
    TreeHash,
    arbitrary::Arbitrary,
)]
#[serde(bound = "T: EthSpec")]
#[arbitrary(bound = "T: EthSpec")]
pub struct ContributionAndProof<T: EthSpec> {
    /// The index of the validator that created the sync contribution.
    #[serde(with = "serde_utils::quoted_u64")]
    pub aggregator_index: u64,
    /// The aggregate contribution.
    pub contribution: SyncCommitteeContribution<T>,
    /// A proof provided by the validator that permits them to publish on the
    /// `sync_committee_contribution_and_proof` gossipsub topic.
    pub selection_proof: Signature,
}

impl<T: EthSpec> ContributionAndProof<T> {
    /// Produces a new `ContributionAndProof` with a `selection_proof` generated by signing
    /// `SyncAggregatorSelectionData` with `secret_key`.
    ///
    /// If `selection_proof.is_none()` it will be computed locally.
    pub fn from_aggregate(
        aggregator_index: u64,
        contribution: SyncCommitteeContribution<T>,
        selection_proof: Option<SyncSelectionProof>,
        secret_key: &SecretKey,
        fork: &Fork,
        genesis_validators_root: Hash256,
        spec: &ChainSpec,
    ) -> Self {
        let selection_proof = selection_proof
            .unwrap_or_else(|| {
                SyncSelectionProof::new::<T>(
                    contribution.slot,
                    contribution.subcommittee_index,
                    secret_key,
                    fork,
                    genesis_validators_root,
                    spec,
                )
            })
            .into();

        Self {
            aggregator_index,
            contribution,
            selection_proof,
        }
    }
}

impl<T: EthSpec> SignedRoot for ContributionAndProof<T> {}
