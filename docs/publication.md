# Citation and archival publication

[CITATION.cff](../CITATION.cff) supplies machine-readable author, license, repository
and package-version metadata. Charles Russella is the name on the repository
owner's public GitHub profile. No DOI or release date is claimed. Version 0.3.0 is
the Cargo package version, not evidence of a published archival release.

## Current integration status

Zenodo activation is **not verified**. Its GitHub settings require an authenticated
account, and the available browser tool failed during initialization. T-017 remains
open. This is an access dependency, not a completed integration or an automatic
approval rejection. No Zenodo credentials or tokens have been requested or stored.
The user explicitly deferred activation to continue implementation. It no longer
blocks G0, but remains required before an archival-release claim.

## Enable and verify the repository

1. Sign in to [Zenodo](https://zenodo.org/login/) using the intended archive owner.
2. Review and authorize the GitHub integration if needed.
3. In [Zenodo's GitHub settings](https://zenodo.org/account/settings/github/), find
   `crussella0129/Project_Lagoon` and enable it.
4. Record the enabled repository and verification date in the work ledger.
   Verify the exact repository, not merely that GitHub login succeeded.

The repository is public, satisfying that integration prerequisite. As described
in [GitHub's archive instructions](https://docs.github.com/en/repositories/archiving-a-github-repository/referencing-and-citing-content),
Zenodo archives a **GitHub release**. A Git tag alone does not cause archival DOI
publication. Release creation remains a separately approved action under the
repository's human-approval workflow.

## Before an approved release

- Ensure the release commit's tests pass; record the actual Rust compiler, lockfile
  and immutable source revision. The support channel is rolling stable.
- Update Cargo and citation versions consistently, and validate CFF against the
  [official 1.2.0 schema](https://github.com/citation-file-format/citation-file-format/blob/1.2.0/schema.json).
- Review the archive contents: no private operator notes, whispers, hidden ballots,
  credentials, restricted checkpoints or datasets. Apply [ETHICS.md](../ETHICS.md).
- Prepare concrete release notes identifying implemented behavior and limitations;
  obtain release approval before publishing the GitHub release.
- After publication, inspect Zenodo's resulting record, creators, license, files,
  source revision and DOI. Retain both version-specific and concept identifiers
  when provided. Do not report success on a queued or failed archival webhook.
- Add the verified DOI to citation metadata and link the version-specific archive.
  Do not rewrite history or claim that a citation-file update changes an already
  published archive. Keep subsequent release metadata consistent.

## Studies and records

OSF registrations freeze study protocols; Zenodo release archives preserve software
or separately approved data artifacts. Neither replaces the other. OpenTimestamps
anchors a digest and requires a verified completed proof before being described as
complete. The Study A/B files remain local drafts until those procedures occur.

Any paper, export dataset or documentary needs a separate rights/privacy review
and an explicit public-artifact manifest. Code publication does not authorize
publication of trusted-operator records. No participant data have been published.
