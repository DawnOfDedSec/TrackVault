import win32com.client
import subprocess
import datetime
import platform

from client.utils import logify


def getInstalledUpdates():
    # Create a Windows Update Session object
    update_session = win32com.client.Dispatch("Microsoft.Update.Session")
    update_searcher = update_session.CreateUpdateSearcher()

    # Search for all installed updates
    search_result = update_searcher.Search("IsInstalled=0 OR IsInstalled=1")

    # Initialize a dictionary to store update details
    updates = []

    # Loop through the installed updates and extract details
    for update in search_result.Updates:
        update_info = {
            "Title": update.Title,
            "Description": update.Description,
            "KBArticleIDs": [kb for kb in update.KBArticleIDs],
            "SupportUrl": update.SupportUrl,
            "IsInstalled": update.IsInstalled,
            "Type": (
                "Security Update" if "Security" in update.Title else "Cumulative Update"
            ),
            "LastDeploymentChangeTime": (
                update.LastDeploymentChangeTime.strftime("%d-%m-%Y")
                if update.LastDeploymentChangeTime
                else None
            ),
            "SecurityBulletinIDs": [sb for sb in update.SecurityBulletinIDs],
            "MsrcSeverity": update.MsrcSeverity,
        }

        updates.append(update_info)

    return updates


def getInstallDate():
    # Get the Windows installation date using systeminfo command
    try:
        result = subprocess.run(
            ["systeminfo"], stdout=subprocess.PIPE, text=True, check=True
        )
        system_info = result.stdout

        # Extract the install date from the systeminfo output
        for line in system_info.splitlines():
            if "Original Install Date" in line:
                install_date_str = line.split(":")[1].strip()
                # Convert to a datetime object
                install_date = datetime.strptime(
                    install_date_str, "%m/%d/%Y, %I:%M:%S %p"
                )
                return install_date.strftime("%d-%m-%Y %H:%M:%S")
    except Exception as e:
        logify.error(f"Error retrieving install date: {e}")
    return "Unknown"


def getWinType():
    # Get the Windows edition (e.g., Pro, Home)
    try:
        result = subprocess.run(
            ["systeminfo"], stdout=subprocess.PIPE, text=True, check=True
        )
        system_info = result.stdout

        # Extract the Windows edition from the systeminfo output
        for line in system_info.splitlines():
            if "OS Name" in line:
                return line.split(":")[1].strip()
    except Exception as e:
        logify.error(f"Error retrieving Windows type: {e}")
    return "Unknown"


def get():

    info = {}

    info["updates"] = getInstalledUpdates()

    info["osName"] = platform.system()

    info["osVersion"] = platform.version()

    info["buildVersion"] = platform.win32_ver()[1]

    info["kernalVersion"] = platform.release()

    info["Type"] = getWinType()

    info["installDate"] = getInstallDate()

    return info
