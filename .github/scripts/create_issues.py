import os
import sys
import json
import requests

# Get the warning messages from the input
warnings = sys.stdin.read()

# Set up the GitHub API
repo = os.getenv("GITHUB_REPOSITORY")
token = os.getenv("GITHUB_TOKEN")
headers = {
    "Accept": "application/vnd.github+json",
    "Authorization": f"token {token}",
}

# Iterate over the warnings and create issues
for warning in warnings.strip().split("\n"):
    # Format the title of the issue
    issue_title = warning.split("\n", 1)[0]
    title = f"Build warning: { issue_title }"

    # Check if an issue with the same title already exists
    search_url = f"https://api.github.com/repos/{repo}/issues?q={title}"
    response = requests.get(search_url, headers=headers)
    existing_issues = json.loads(response.text)

    if len(existing_issues) == 0:
        # Create the issue
        issue_data = {
            "title": title,
            "body": f"```\n{warning}\n```",
            "labels": ["build-warning"],
        }
        create_url = f"https://api.github.com/repos/{repo}/issues"
        requests.post(create_url, headers=headers, json=issue_data)
