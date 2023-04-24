import os
import re
import sys
import json
import requests

if __name__ == "__main__":
    warnings = sys.stdin.read().strip()
    if not warnings:
        sys.exit(0)

    token = os.environ["GITHUB_TOKEN"]
    repo = os.environ["GITHUB_REPOSITORY"]
    api_url = f"https://api.github.com/repos/{repo}/issues"

    headers = {
        "Authorization": f"token {token}",
        "Accept": "application/vnd.github+json",
    }

    warning_list = re.split(r"\n(?=warning:)", warnings)

    for warning in warning_list:
        title = f"Build warning: { warning.split('\n', 1)[0] }"
        body = f"```rust\n{warning}\n```"

        data = {
            "title": title,
            "body": body,
            "labels": ["build-warning"],
        }

        response = requests.post(api_url, headers=headers, json=data)
        if response.status_code != 201:
            print(f"Failed to create issue: {response.text}")
