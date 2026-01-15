#!/bin/bash
# Fetch GitHub contributions for a specific month using GraphQL

USER="${1:-jmikedupont2}"
YEAR="${2:-2024}"
MONTH="${3:-1}"

# Calculate date range
FROM="${YEAR}-$(printf '%02d' $MONTH)-01T00:00:00Z"
if [ "$MONTH" -eq 12 ]; then
  TO="$((YEAR+1))-01-01T00:00:00Z"
else
  TO="${YEAR}-$(printf '%02d' $((MONTH+1)))-01T00:00:00Z"
fi

echo "Fetching contributions for $USER: $FROM to $TO"

gh api graphql -f query='
query($login: String!, $from: DateTime!, $to: DateTime!) {
  user(login: $login) {
    contributionsCollection(from: $from, to: $to) {
      totalCommitContributions
      totalPullRequestContributions
      totalIssueContributions
      totalPullRequestReviewContributions
      contributionCalendar {
        totalContributions
        weeks {
          contributionDays {
            date
            contributionCount
          }
        }
      }
    }
  }
}' -f login="$USER" -f from="$FROM" -f to="$TO"
