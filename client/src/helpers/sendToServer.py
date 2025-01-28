import requests
import time
import os
import sys

sys.path.append(os.path.join(os.path.dirname(__file__), "..", ".."))

from utils import configify, logify
from src import getInfo


# Function to send a POST request
def sendPostInfo():
    try:
        _config = configify.get()
        url = _config["server"]["url"]
        data = getInfo.get()
        headers = {
            "Content-Type": "application/json",
            "Authorization": _config["oAuthToken"],
        }
        requests.post(url, json=data, headers=headers)
        return True

    except requests.exceptions.RequestException as e:
        logify.error(str(e))
        return None


# Timer-based loop to send POST requests
def httpPostLoop(interval=60):
    while True:
        sendPostInfo()
        time.sleep(interval)
