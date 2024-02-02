# Contributing to utapi-rs

Thank you for considering contributing to `utapi-rs`! This document contains guidelines for contributing and setting up the development environment.

## Scope

The `utapi-rs` crate mirrors the functionalities of the [UTApi class in Uploadthing](https://github.com/pingdotgg/uploadthing/blob/main/packages/uploadthing/src/sdk/index.ts#L39). Our goal is to maintain parity with the original TypeScript SDK, focusing on feature consistency and reliability.

## utapi-rs features

The following functionalities are implemented in this crate, as in the original UTApi class:

- [x] `request_uploadthing`(requestUploadThing)
- [x] `delete_files`(deleteFiles)
- [x] `get_file_urls`(getFileUrls)
- [x] `list_files`(listFiles)
- [x] `rename_files`(renameFiles)
- [x] `get_presigned_url`(getSignedURL)
- [x] `get_usage_info`(getUsageInfo)

## Development Setup

### 1. Original Uploadthing SDK Reference

For any modifications or to understand the underlying functionality, start by referencing the original UTApi class from the Uploadthing TypeScript SDK.

```bash
git clone --depth 1 https://github.com/pingdotgg/uploadthing uploadthing_source
cd uploadthing_source
# Recommended starting point
vim packages/uploadthing/src/sdk/index.ts
```

### 2. Clone utapi-rs Repository

Clone the `utapi-rs` repository and create a new branch for your feature or fix.

```bash
git clone https://github.com/ielm/utapi-rs utapi-rs
cd utapi-rs
git checkout -b my_feature
```

### 3. Make Changes

After making and testing your changes...

### 4. Check for Upstream Changes

Ensure your branch is up to date with any recent changes to the main branch.

```bash
git fetch origin
git rebase origin/main
```

### 5. Push Your Branch

Once your changes are ready and rebased:

```bash
git push origin my_feature
```

### 6. Open a Pull Request

Go to GitHub and create a pull request for your branch. Make sure to explain your changes and link to any relevant issues or discussions.

Wait for a review and possible test of your changes.

## Before Contributing...

If you have modifications you'd like to make for cleanliness or other improvements, it's recommended to open an issue or get in touch before making the changes. This will help us discuss and plan the modifications in a way that aligns with the project's goals and roadmap.

## License

By contributing to `utapi-rs`, you agree that your contribution will be licensed under its MIT License.
