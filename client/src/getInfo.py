import subprocess
import json
import datetime
import psutil
import win32com.client
import socket
import re
import os
import sys
import requests
from tzlocal import get_localzone

sys.path.append(os.path.join(os.path.dirname(__file__), "..", ".."))

from controllers import (
    getAssetInfo,
    getClockSyncInfo,
    getControlOfInternet,
    getApplicationsInfo,
    getWinUpdatesInfo,
)

from client.utils import configify, logify


def getSystemInfo():
    try:
        # Use systeminfo command to get system information
        result = subprocess.run(
            ["systeminfo"], capture_output=True, text=True, check=True
        )

        # Extract manufacturer, model, and serial number using regex
        manufacturer = re.search(r"System Manufacturer:\s*(.*)", result.stdout)
        model = re.search(r"System Model:\s*(.*)", result.stdout)
        serial_number = re.search(r"System Serial Number:\s*(.*)", result.stdout)
        domainType = re.search(r"Domain:\s*(.*)", result.stdout)
        domainName = re.search(r"Logon Server:\s*(.*)", result.stdout)

        # Create a dictionary with the extracted information
        sysInfo = {
            "manufacturer": (
                manufacturer.group(1).strip() if manufacturer else "Unknown"
            ),
            "model": model.group(1).strip() if model else "Unknown",
            "serialNumber": (
                serial_number.group(1).strip() if serial_number else "Unknown"
            ),
            "domainName": domainName.group(1).strip() if domainName else "Unknown",
            "domainType": domainType.group(1).strip() if domainType else "Unknown",
        }

        return sysInfo

    except subprocess.CalledProcessError as e:
        logify.error(e)
        return {}


def getNetworkAdapterInfo():
    interfaces = []

    # Get network interface details
    for name, addresses in psutil.net_if_addrs().items():

        x = {"adapter": name, "MAC": None, "IPv4": None, "IPv6": None}

        for address in addresses:
            if address.family == socket.AF_INET:  # IPv4
                x["IPv4"] = address.address
            elif address.family == socket.AF_INET6:  # IPv6
                x["IPv6"] = address.address
            elif address.family == psutil.AF_LINK:  # MAC address
                x["MAC"] = address.address

        interfaces.append(x)

    return interfaces


def getTimeZone():
    # Get the local timezone
    local_timezone = get_localzone()
    return str(local_timezone)


def getPublicIPAddress():
    # Use a public API to get the public IP address
    try:
        response = requests.get("https://api.ipify.org?format=json")
        response.raise_for_status()
        data = response.json()
        return data["ip"]
    except requests.RequestException as e:
        logify.error(e)
        return None


def saveToJson():
    info = get()
    with open(
        os.path.join(os.path.dirname(__file__), "..", "test", "getInfo-output.json"),
        "w",
    ) as f:
        f.write(info.dumps(info, indent=4))


def get():
    info = {}

    oemInfo = getSystemInfo()
    _config = configify.get()

    # Basic system info
    info["hostName"] = socket.gethostname()
    info["domainName"] = oemInfo["domainName"]
    info["domainType"] = oemInfo["domainType"]
    info["userName"] = os.getlogin()
    info["timeZone"] = getTimeZone()
    info["publicIpAddress"] = getPublicIPAddress()
    info["dataFetchedTime"] = datetime.datetime.now().strftime("%d-%m-%Y %H:%M:%S")
    info["networkInfo"] = getNetworkAdapterInfo()

    wmi = win32com.client.GetObject("winmgmts:")

    info["assetInfo"] = getAssetInfo.get(wmi)
    info["clockSyncInfo"] = getClockSyncInfo.get(info["timeZone"])

    info["controlOfInternetInfo"] = getControlOfInternet.get(_config)

    info["installedApplications"] = getApplicationsInfo.get(wmi, _config)

    info["installedWinUpdates"] = getWinUpdatesInfo.get()

    return json
