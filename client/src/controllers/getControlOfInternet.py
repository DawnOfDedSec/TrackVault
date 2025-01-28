import requests
from urllib.parse import urlparse
import random
import requests


from client.utils import logify, configify
from utils.coreUtils import isDictEx

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


def updateGlobalUrls(config):
    try:
        # Use a public API to get the current time
        res = requests.get(
            "https://raw.githubusercontent.com/stamparm/blackbook/refs/heads/master/blackbook.txt"
        )

        data = res.text.split("\n")
        malUrls = []

        for line in data:
            malUrls.append("https://" + line + "/")
            malUrls.append("http://" + line + "/")

        if isDictEx(config, "static.maliciousUrls"):
            oldMalUrls = config["static"]["maliciousUrls"]
        else:
            oldMalUrls = []

        if len(malUrls) > 10:
            malUrls = random.sample(malUrls, 10)

        if isDictEx(config, "static.maxMaliciousUrlsCheck"):
            maxUrlsCheck = config["static"]["maxMaliciousUrlsCheck"]
        else:
            maxUrlsCheck = len(oldMalUrls) + 10

        urls = list(set(oldMalUrls + malUrls))[:maxUrlsCheck]

        configify.modify("dynamic.fetchedMaliciousUrls", malUrls)

        return urls

    except requests.RequestException as e:
        logify.error(str(e))
        return []


def get(config):

    if isDictEx(config, "options.autoUpdateMaliciousUrls", "true"):
        urls = updateGlobalUrls(config)

    elif isDictEx(config, "static.maliciousUrls"):
        urls = config["static"]["maliciousUrls"]

    else:
        return None

    results = []

    for url in urls:
        is_blocked, blocked_by = getBlockStatus(url)
        results.append(
            {
                "url": url,
                "status": "Blocked" if is_blocked else "Accessible",
                "blockedBy": blocked_by if is_blocked else None,
            }
        )

    return results
