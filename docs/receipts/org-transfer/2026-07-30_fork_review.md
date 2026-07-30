# Maintained-fork ownership review

**Reviewed:** 2026-07-30
**Decision:** Keep ten thin forks under `mark-ik`; admit Vano to
`merely-made`.

## Boundary decision

Fork ancestry alone does not decide organization ownership. A downstream fork
stays under the personal owner while it is a thin patch carrier, mirror, or
evaluation checkout. It becomes a Merely Made repository when both conditions
hold:

1. the fork is integral to a Merely platform or product; and
2. its maintained divergence has become a meaningful project surface.

Vano meets that threshold. Its data-oriented engine architecture is part of
Genet's framing, and its downstream line carries substantial embedder,
weak-liveness, Wasm64, scheduling, and conformance work. The other ten reviewed
forks remain under `mark-ik`.

## Live fork evidence

Ahead and behind counts compare the named downstream branch with the current
upstream default branch at review time.

| Fork | Upstream | Reviewed branch | Ahead / behind | Decision |
| --- | --- | --- | ---: | --- |
| `mark-ik/arboard` | `1Password/arboard` | `custom-formats` at `d57a91cc` | 5 / 2 | Keep personal patch carrier |
| `mark-ik/boa` | `boa-dev/boa` | `genet` at `114c45a9` | 2 / 27 | Keep personal patch carrier |
| `mark-ik/emissary` | `eepnet/emissary` | `master` at `30ef8774` | 1 / 0 | Keep personal evaluation patch |
| `mark-ik/iroh` | `n0-computer/iroh` | `main` at `117ae31c` | 0 / 4 | Keep personal upstream mirror |
| `mark-ik/iroh-address-lookups` | `n0-computer/iroh-address-lookups` | `mere` at `1f7cfe85` | 3 / 2 | Keep personal patch carrier |
| `mark-ik/p2panda` | `p2panda/p2panda` | `main` at `44e067d8` | 1 / 3 | Keep personal patch carrier |
| `mark-ik/piccolo` | `kyren/piccolo` | `master` at `e77309c6` | 1 / 0 | Keep personal patch carrier |
| `mark-ik/stylo` | `servo/stylo` | `main` at `b157d925` | 39 / 194 | Keep personal pending a clearer independent surface |
| `mark-ik/swarm-discovery` | `rkuhn/swarm-discovery` | `mere` at `99dd24c5` | 1 / 0 | Keep personal patch carrier |
| `mark-ik/vano` | `trynova/nova` | `genet-embedder` at `87ef049a` before project commit | 18 / 0 | Transfer as Merely foundation |
| `mark-ik/xilem` | `linebender/xilem` | `main` at `c5950bcb` | 0 / 2 | Keep personal; former Woodshed branches retired |

All eleven GitHub repositories reported the listed upstream fork ancestry.
`merely-made` had no target-name or fork-network collision for any of them.

## License and provenance

Root license files, Cargo package declarations, and GitHub fork ancestry were
reviewed. The resulting ledger records:

- Arboard, Iroh, Iroh Address Lookups, and P2panda:
  `MIT OR Apache-2.0`;
- Boa: `Unlicense OR MIT`;
- Emissary: `MIT`;
- Piccolo: `MIT OR CC0-1.0`;
- Stylo: MPL-2.0 for the Stylo family, with inherited MIT or Apache-2.0
  support crates;
- Swarm Discovery: `Apache-2.0`;
- Vano: `MPL-2.0`;
- Xilem: `Apache-2.0`.

The review confirms provenance as a GitHub fork of each named upstream. It
does not turn inherited upstream history into a first-party authorship claim.

## Sensitive-information review

Gitleaks 8.30.1 scanned archive exports of the active downstream trees and
the commit ranges unique to their reviewed branches. The fork-only ranges
reported zero findings.

Current trees also reported:

- zero local profile paths;
- zero local machine-name matches;
- zero configured contact-address matches;
- zero first-party private endpoints.

P2panda produced two generic-key detector matches in inherited test variable
names. They are identifiers, not key material. RFC 1918 matches elsewhere
were standards numbering, tests, examples, or protocol fixtures. No tracked
machine-local file required removal or a new ignore rule.

## Vano admission

Vano's `main` was a clean ancestor of `genet-embedder`. The reviewed
integration line was fast-forwarded to `main`, then commit `1816f328`
established the Vano project identity, organization metadata, and Genet
terminology.

The publication gate reported:

- public MPL-2.0 fork of `trynova/nova`;
- zero repository or environment secrets and variables;
- zero deploy keys, webhooks, Pages sites, releases, or rulesets;
- two Actions workflows with Actions enabled;
- no target-name or fork-network collision;
- pushed, recoverable default branch `main`.

GitHub transferred the repository to `merely-made/vano`. The former
`mark-ik/vano` and older `mark-ik/nova` web and Git URLs redirect to the
organization repository and resolve the same `main` head. The separate
[`vano.md`](vano.md) receipt records the transfer verification.

Genet now resolves `nova_vm` from `merely-made/vano` while preserving locked
commit `22b4298960b4794cc3064d4676e3470c850c6275`. Locked metadata and
`cargo check -p script-engine-nova --locked --offline` passed from outside the
workspace config hierarchy.

## Result

Ten records use `disposition = "keep-personal"`. Vano is a first-party
foundation repository in `merely-made` and an explicit dependency of Genet in
the public repository graph.

No other fork transferred, archived, deleted, or rewrote history. Re-run this
same significance test when another fork becomes integral enough to present an
independent Merely project surface.
