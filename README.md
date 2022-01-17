## netlify-rust-functions

Example of setting up a Rust workspace to build out lambda functions using Rust on Netlify.

### Deploy from Github Actions instead (optional)

This repository does have the ability to deploy from Github Actions instead, but is not needed now for Netlify.

The un-needed files are:

- `.github/workflows/deploy-on-push.yml`
- `./Makefile` (used for a build command `make build`)
- You would also need two secret key values in your repository `NETLIFY_AUTH_TOKEN` and `NETLIFY_SITE_ID` to be able to have the action work.

**NOTE:** Delete the 2 files above OR disable actions for this repository to deploy on Netlify
