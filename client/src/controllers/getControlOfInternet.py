import requests
from urllib.parse import urlparse
import sys
import os

sys.path.append(os.path.join(os.path.dirname(__file__), "..", "..", "utils"))

from client.utils import logify

# List of known block pages (you can expand this list)
KNOWN_BLOCK_PAGES = [
    "sophos.com",
    "fortinet.com",
]


def getBlockStatus(url):
    try:
        response = requests.get(url, timeout=5)
        # Check if the final URL is a known block page
        final_url = response.url
        parsed_final_url = urlparse(final_url)
        if any(
            block_page in parsed_final_url.netloc for block_page in KNOWN_BLOCK_PAGES
        ):
            return True, "Antivirus"
        return False, None
    except requests.exceptions.RequestException as e:
        # If there's a connection error, assume it's blocked by a firewall
        return True, "Firewall"


def get(urls):
    results = {}

    for url in urls:
        is_blocked, blocked_by = getBlockStatus(url)
        results[url] = {
            "status": "Blocked" if is_blocked else "Accessible",
            "blockedBy": blocked_by if is_blocked else None,
        }

    return results
