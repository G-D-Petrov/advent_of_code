import os
import sys
import json
import re
from github import Github

# Read the log file specified as a command-line argument
log_file = sys.argv[1]
with open(log_file, "r") as f:
    lines = f.readlines()

print("Creating issues for build warnings...")
print(f"log file: {lines}")

# Filter the lines to get only warnings
warnings = [line.strip() for line in lines if line.startswith("warning:")]

# Initialize GitHub API client
gh = Github(os.environ["GITHUB_TOKEN"])
repo = gh.get_repo(os.environ["GITHUB_REPOSITORY"])

# Create issues for each warning
for warning in warnings:
    # Extract the warning message without the newline character
    title = warning.split("\n", 1)[0].replace("\\", "")

    # Check if an issue with the same title already exists
    existing_issues = repo.get_issues(state="open", labels=["build-warning"])
    issue_exists = any(issue.title == title for issue in existing_issues)

    if not issue_exists:
        # Create a new issue
        issue = repo.create_issue(
            title=title,
            body=f"**Warning message:**\n```\n{warning}\n```",
            labels=["build-warning"],
        )
        print(f"Created issue: {issue.number} {issue.html_url}")
