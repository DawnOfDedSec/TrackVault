import platform
import socket
import datetime
import psutil
import re
import os
import subprocess

from client.utils import logify


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


def getRAMsInfo(wmi):

    # Query for RAM information
    ramInfo = wmi.ExecQuery("SELECT * FROM Win32_PhysicalMemory")

    ramTypes = {
        20: "DDR",
        21: "DDR2",
        22: "DDR2 FB-DIMM",
        24: "DDR3",
        26: "DDR4",
        27: "LPDDR4",
        28: "LPDDR4X",
        29: "LPDDR5",
        30: "DDR5",
    }

    # Iterate through the RAM modules and extract the required information
    rams = []
    for ram in ramInfo:
        rams.append(
            {
                "SerialNumber": ram.SerialNumber.strip(),
                "Model": ram.PartNumber.strip(),
                "Manufacturer": ram.Manufacturer.strip(),
                "TotalSize": (int(ram.Capacity) // (1024**3)),  # Convert bytes to GB
                "Type": ramTypes.get(ram.SMBIOSMemoryType, "Unknown Type"),
            }
        )

    return rams


def getDiskInfo(wmi):

    # Query for physical disks
    rDiskInfo = wmi.ExecQuery("SELECT * FROM Win32_DiskDrive")
    disks = []

    for d in rDiskInfo:
        x = {
            "SerialNumber": d.SerialNumber.strip() if d.SerialNumber else "Unknown",
            "Model": d.Model.strip() if d.Model else "Unknown",
            "Manufacturer": d.Manufacturer.strip() if d.Manufacturer else "Unknown",
            "TotalSize": (int(d.Size) // (1024**3)) if d.Size else 0,
        }

        if d.MediaType == "Fixed hard disk media":
            if "NVMe" in d.Model:
                x["type"] = "NVMe SSD"
            elif "SSD" in d.Model:
                x["type"] = "SATA SSD"
            elif "HDD" in d.Model:
                x["type"] = "HDD"
            else:
                x["type"] = "Unknown"
        else:
            x["type"] = "Unknown"

        disks.append(x)

    return disks


def getMotherboardInfo(wmi):
    motherboard_info = {}

    # Query for motherboard information
    for item in wmi.ExecQuery("SELECT * FROM Win32_BaseBoard"):
        motherboard_info["Manufacturer"] = item.Manufacturer
        motherboard_info["Product"] = item.Product
        motherboard_info["SerialNumber"] = item.SerialNumber
        motherboard_info["Version"] = item.Version

    return motherboard_info


def getBiosInfo(wmi):
    bios_info = {}

    # Query for BIOS information
    for item in wmi.ExecQuery("SELECT * FROM Win32_BIOS"):
        bios_info["Manufacturer"] = item.Manufacturer
        bios_info["Name"] = item.Name
        bios_info["SerialNumber"] = item.SerialNumber
        bios_info["Version"] = item.Version
        bios_info["ReleaseDate"] = datetime.datetime.strptime(
            item.ReleaseDate.split(".")[0], "%Y%m%d%H%M%S"
        ).strftime("%d-%m-%Y")

    return bios_info


def getOSInfo(wmi):
    osInfo = {}

    # Get OS platform and version
    osInfo["platform"] = platform.system()
    osInfo["version"] = platform.version()
    osInfo["release"] = platform.release()

    # Get OS edition and more detailed version info
    for os in wmi.InstancesOf("Win32_OperatingSystem"):
        osInfo["edition"] = os.Caption
        osInfo["build"] = os.BuildNumber
        osInfo["installDate"] = datetime.datetime.strptime(
            os.InstallDate.split(".")[0], "%Y%m%d%H%M%S"
        ).strftime("%d-%m-%Y %H:%M:%S")
        osInfo["lastBootTime"] = datetime.datetime.strptime(
            os.LastBootUpTime.split(".")[0], "%Y%m%d%H%M%S"
        ).strftime("%d-%m-%Y %H:%M:%S")

    return osInfo


def getGroupMembers(name):
    command = f'net localgroup "{name}"'
    result = subprocess.run(
        command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True
    )

    members = []
    lines = result.stdout.splitlines()

    for i, line in enumerate(lines):
        if "----" in line.strip():
            start_index = i + 1
            break

    for line in lines[start_index:]:
        if line.strip() == "The command completed successfully.":
            break
        member = line.strip()
        if member:
            members.append(member)

    return members


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


def get(wmi):

    oemInfo = getSystemInfo()
    logify.log(
        "Asset-Extractor | System Information Command Data Value has been Fetched & Parsed"
    )

    # Detailed info
    assetInfo = {}

    assetInfo["hostName"] = socket.gethostname()
    assetInfo["userName"] = os.getlogin()
    assetInfo["domainName"] = oemInfo["domainName"]
    assetInfo["domainType"] = oemInfo["domainType"]
    assetInfo["manufacturer"] = oemInfo["manufacturer"]
    assetInfo["model"] = oemInfo["model"]
    assetInfo["serialNumber"] = oemInfo["serialNumber"]
    assetInfo["networkInfo"] = getNetworkAdapterInfo()

    logify.log(
        "Asset-Extractor | SystemInformation Parsed for Asset Info for Excel Export"
    )

    # RAM Info
    assetInfo["ramInfo"] = getRAMsInfo(wmi)

    # Partitions Info
    rPartitions = psutil.disk_partitions()
    partitionInfo = []
    for p in rPartitions:
        usage = psutil.disk_usage(p.mountpoint)
        partitionInfo.append(
            {
                "diskName": p.device,
                "mountPoint": p.mountpoint,
                "type": p.fstype,
                "totalSize": usage.total // (1024**3),
                "usedSize": usage.used // (1024**3),
                "freeSize": usage.free // (1024**3),
                "usedPercent": usage.percent,
            }
        )

    logify.log(
        "Asset-Extractor | Partition Info from Storage has been Fetched & Parsed"
    )

    assetInfo["storageInfo"] = {"disks": getDiskInfo(wmi), "partitions": partitionInfo}

    assetInfo["motherboardInfo"] = getMotherboardInfo(wmi)
    logify.log("Asset-Extractor | Motherboard Info has been Fetched & Parsed")

    assetInfo["biosInfo"] = getBiosInfo(wmi)
    logify.log("Asset-Extractor | Bios Info has been Fetched & Parsed")

    # Operating System Info
    assetInfo["operatingSystem"] = getOSInfo(wmi)
    logify.log("Asset-Extractor | Operating System Info has been Fetched & Parsed")

    # Administrator Group Members
    assetInfo["localAdministrators"] = getGroupMembers("administrators")
    logify.log(
        "Asset-Extractor | Local Administrator Member Names has been Fetched & Parsed"
    )

    # Remote Desktop Users members
    assetInfo["localRemoteDesktopUsers"] = getGroupMembers("remote desktop users")
    logify.log("Asset-Extractor | Local Remote Desktop Users has been Fetched & Parsed")

    return assetInfo
