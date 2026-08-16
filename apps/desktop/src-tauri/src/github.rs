// GitHub REST API client.
//
// Responsibilities (to be implemented):
//   - Fetch open PRs where the authenticated user is a requested reviewer
//   - Fetch PR metadata (title, author, description, base/head branches)
//   - Fetch per-file diffs for a given PR
//   - Submit a review (with inline comments) to GitHub
//
// The GitHub personal access token will be retrieved from the OS keychain,
// never stored in plaintext or passed through the frontend.
