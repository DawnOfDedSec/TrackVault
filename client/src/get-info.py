import subprocess
import json
import datetime
import psutil
import win32com.client
import socket
import re
import os
import sys

sys.path.append(os.path.join(os.path.dirname(__file__), "..", ".."))

from controllers import (
    getInstalledApps,
    getAssetInfo,
    getClockSyncInfo,
    getControlOfInternet,
)
from client.utils import configify


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
        print(e)
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


def get():
    info = {}

    oemInfo = getSystemInfo()
    _config = configify.get()

    # Basic system info
    info["hostName"] = socket.gethostname()
    info["domainName"] = oemInfo["domainName"]
    info["domainType"] = oemInfo["domainType"]
    info["userName"] = os.getlogin()
    info["dataFetchedTime"] = datetime.datetime.now().strftime("%d-%m-%Y %H:%M:%S")
    info["networkInfo"] = getNetworkAdapterInfo()

    wmi = win32com.client.GetObject("winmgmts:")

    info["assetInfo"] = getAssetInfo.get(wmi)
    info["clockSyncInfo"] = getClockSyncInfo.get(wmi)

    mUrls = _config["static"]["maliciousUrls"]
    # info["controlOfInternetInfo"] = getControlOfInternet.get(mUrls)

    # info["installedApplications"] = getInstalledApps.get(wmi)

    return json.dumps(info, indent=4)


print(get())
