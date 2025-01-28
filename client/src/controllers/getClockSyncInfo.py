import subprocess
import requests
import datetime
import ntplib
import psutil
import socket
import re
import os

from client.utils import logify


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
        logify.error(str(e))
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


def getNtpInfo():
    try:
        # Execute the w32tm command to get NTP status
        result = subprocess.run(
            ["w32tm", "/query", "/status"], capture_output=True, text=True, check=True
        )
        output = result.stdout

        # Split the output into lines
        lines = output.strip().split("\n")

        # Create a dictionary to store NTP details
        ntp = {}

        for line in lines:
            # Skip empty lines
            if not line.strip():
                continue

            # Split the line into key and value based on the first colon
            parts = line.split(":", 1)
            if len(parts) >= 2:
                key = parts[0].strip()
                value = parts[1].strip()
                ntp[key] = value

        return ntp

    except subprocess.CalledProcessError as e:
        logify.error(str(e))
        return None


def getWorldApiTime(timeZone="Asia/Kolkata"):

    try:
        # Use a public API to get the current time
        res = requests.get(
            "https://www.timeapi.io/api/time/current/zone?timeZone=" + timeZone
        )
        data = res.json()
        return datetime.datetime.fromisoformat(data["dateTime"])

    except requests.RequestException as e:
        logify.error(str(e))
        return None


def getNtpPoolTime():
    # Get the actual time from an NTP server
    ntp_client = ntplib.NTPClient()
    response = ntp_client.request("time.windows.com")
    return datetime.datetime.fromtimestamp(response.tx_time)


def getSystemTime():
    # Get the current system time
    return datetime.datetime.now()


def getNtpPeers():
    # Run the w32tm command to query NTP peers
    result = subprocess.run(
        ["w32tm", "/query", "/peers"], capture_output=True, text=True
    )

    # Check if the command was successful
    if result.returncode != 0:
        return None

    # Parse the output to extract NTP peers details
    ntp_peers = []
    lines = result.stdout.splitlines()

    for line in lines:
        # Match lines that contain NTP peer information
        match = re.match(
            r"^\s*#(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s*$",
            line,
        )
        if match:
            peer_info = {
                "index": int(match.group(1)),
                "peer_address": match.group(2),
                "ref_id": match.group(3),
                "st": int(match.group(4)),
                "when": int(match.group(5)),
                "poll": int(match.group(6)),
                "reach": int(match.group(7)),
                "delay": float(match.group(8)),
                "offset": float(match.group(9)),
            }
            ntp_peers.append(peer_info)

    return ntp_peers


def get( timeZone):
    info = {}

    oemInfo = getSystemInfo()

    # Basic system info
    info["hostName"] = socket.gethostname()
    info["domainName"] = oemInfo["domainName"]
    info["domainType"] = oemInfo["domainType"]
    info["userName"] = os.getlogin()
    info["networkInfo"] = getNetworkAdapterInfo()

    # Get NTP details
    info["ntpStatusInfo"] = getNtpInfo()

    worldApiTime = getWorldApiTime(timeZone)
    if worldApiTime:
        # Fetch actual time from the API
        info["worldApiTime"] = worldApiTime.strftime("%d-%m-%Y %H:%M:%S")

    ntpPoolTime = getNtpPoolTime()
    if ntpPoolTime:
        # Fetch actual time from the API
        info["ntpPoolTime"] = ntpPoolTime.strftime("%d-%m-%Y %H:%M:%S")

    curSystemTime = getSystemTime()
    if curSystemTime:
        # Get the system time
        info["currentSystemTime"] = curSystemTime.strftime("%d-%m-%Y %H:%M:%S")

    # Calculate the time difference
    if worldApiTime:
        info["timeDifference"] = str(
            datetime.timedelta(-1, 0, 0, 0, 0, 24) - (curSystemTime - worldApiTime)
        )
    elif ntpPoolTime:
        info["timeDifference"] = str(
            datetime.timedelta(-1, 0, 0, 0, 0, 24) - (curSystemTime - ntpPoolTime)
        )
    else:
        info["timeDifference"] = "N/A"

    # Get NTP Peers details
    info["ntpPeers"] = getNtpPeers()

    return info
