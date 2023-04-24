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

# Get all open issues
issues_url = f"https://api.github.com/repos/{repo}/issues?state=open"
response = requests.get(issues_url, headers=headers)
open_issues = json.loads(response.text)

print(f"Found {len(open_issues)} open issues")

# Iterate over the warnings and create issues
for warning in warnings.strip().split("\n"):
    # Format the title of the issue
    print(f"Creating issue for warning: {warning}")
    issue_title = warning.split('\n', 1)[0]
    title = f"Build warning: { issue_title }"

    # Check if an issue with the same title already exists
    existing_issue = None
    for issue in open_issues:
        if issue["title"] == title:
            existing_issue = issue
            break

    if existing_issue is None:
        # Create the issue
        issue_data = {
            "title": title,
            "body": f"```\n{warning}\n```",
            "labels": ["build-warning"],
        }
        create_url = f"https://api.github.com/repos/{repo}/issues"
        res = requests.post(create_url, headers=headers, json=issue_data)
        print(f"Created issue: {res.status_code} {res.text}")
