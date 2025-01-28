import datetime
import os
import requests
import random

from client.utils import logify, configify
from utils.coreUtils import isDictEx


def getInstalledApps(wmi):
    colInstApps = []

    instApps = wmi.ExecQuery("SELECT * FROM Win32_Product")

    for app in instApps:
        cApps = {}

        # Application name
        cApps["name"] = app.Name

        cApps["caption"] = app.Caption

        # Version
        cApps["version"] = app.Version

        cApps["description"] = app.Description

        cApps["vendor"] = app.Vendor

        # Install location
        cApps["installLocation"] = app.InstallLocation

        cApps["installSource"] = app.InstallSource

        # Install date - If present, convert to readable format
        if app.InstallDate:
            try:
                instDate = datetime.datetime.strptime(str(app.InstallDate), "%Y%m%d")
                cApps["installDate"] = instDate.strftime("%d-%m-%Y")
            except Exception as e:
                logify.error(e)
                cApps["installDate"] = None

        colInstApps.append(cApps)

    return colInstApps


def getLocalDrives():
    # Get all drive letters on the system
    drives = []
    for letter in "ABCDEFGHIJKLMNOPQRSTUVWXYZ":
        drive = f"{letter}:\\"
        if os.path.exists(drive):
            # Check if the drive is a local drive
            try:
                drive_type = os.stat(drive).st_dev
                if drive_type != 46:  # Network drives have a different st_dev value
                    drives.append(drive)
            except OSError:
                continue
    return drives


def searchExeFiles(fileNames):
    drives = getLocalDrives()

    for drive in drives:
        for root, dirs, files in os.walk(drive):
            for file in files:

                if any(f.lower() in file.lower() for f in fileNames):
                    file_path = os.path.join(root, file)

                    try:
                        created_time = os.path.getctime(file_path)
                        modified_time = os.path.getmtime(file_path)

                        created_date = datetime.datetime.fromtimestamp(
                            created_time
                        ).strftime("%Y-%m-%d %H:%M:%S")

                        modified_date = datetime.datetime.fromtimestamp(
                            modified_time
                        ).strftime("%Y-%m-%d %H:%M:%S")

                        info = {
                            "Name": file,
                            "Location": file_path,
                            "Created Date": created_date,
                            "Last Modified Date": modified_date,
                        }
                        return info

                    except Exception as e:
                        logify.error(f"Error accessing {file_path}: {e}")

    return None


def updateGlobalFileNames(config):
    try:
        # Use a public API to get the current time
        res = requests.get(
            "https://raw.githubusercontent.com/certtools/malware_name_mapping/refs/heads/master/malpedia.csv"
        )

        data = res.text.split("\n")
        malwareNames = []

        for line in data:
            if len(line.split(",")) > 1:
                malwareNames.append(line.split(",")[1])

        if isDictEx(config, "static.suspiciousFiles"):
            oldFileNames = config["static"]["suspiciousFiles"]
        else:
            oldFileNames = []

        if len(malwareNames) > 10:
            malwareNames = random.sample(malwareNames, 10)

        if isDictEx(config, "static.maxSuspiciousFilesCheck"):
            maxSusFilesCheck = config["static"]["maxSuspiciousFilesCheck"]
        else:
            maxSusFilesCheck = len(oldFileNames) + 10

        malwares = list(set(oldFileNames + malwareNames))[:maxSusFilesCheck]

        configify.modify("dynamic.fetchedSuspiciousFiles", malwareNames)

        return malwares

    except requests.RequestException as e:
        logify.error(str(e))
        return []


def get(wmi, config):

    applicationsInfo = {}
    fileNames = []

    applicationsInfo["installedApps"] = getInstalledApps(wmi)

    if isDictEx(
        config,
        "options.autoUpdateSuspiciousFiles",
        "true",
    ):
        fileNames = updateGlobalFileNames(config)

    elif isDictEx(
        config,
        "static.suspiciousFiles",
    ):
        fileNames = config["static"]["suspiciousFiles"]

    if len(fileNames) > 0:
        applicationsInfo["suspiciousExes"] = searchExeFiles(fileNames)

    return applicationsInfo
