// Local SQLite database.
//
// Responsibilities (to be implemented):
//   - Run migrations on startup to create tables if they don't exist
//   - Persist cached PR metadata and AI summaries (avoid re-fetching)
//   - Store draft comments locally until the user submits their review
//
// Schema (planned):
//   prs           — cached PR data, AI summary, danger score, review status
//   draft_comments — comments written by the user, with level tags, before submission
