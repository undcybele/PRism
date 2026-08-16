// Claude API client.
//
// Responsibilities (to be implemented):
//   - Accept a PR diff + metadata as input
//   - Send a structured prompt to the Claude API
//   - Return a parsed response containing:
//       - Plain-language summary (3-5 sentences)
//       - Danger score (1-10) with rationale
//       - Suggested areas to focus the review on
//
// The Claude API key will be retrieved from the OS keychain.
