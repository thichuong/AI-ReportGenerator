"""
Google Indexing API Service
Notify Google when new reports are published.

This module handles sending URL_UPDATED notifications to Google Indexing API
to request immediate indexing of newly created report pages.
"""

import json
import os
from typing import Optional

try:
    from google.oauth2 import service_account
    from googleapiclient.discovery import build
    from googleapiclient.errors import HttpError
    GOOGLE_API_AVAILABLE = True
except ImportError:
    GOOGLE_API_AVAILABLE = False
    print("Warning: google-auth or google-api-python-client not installed. Google Indexing disabled.")

# Google Indexing API scope
SCOPES = ["https://www.googleapis.com/auth/indexing"]


def _get_credentials_path() -> Optional[str]:
    """
    Get the path to service account credentials.

    Priority:
    1. GOOGLE_APPLICATION_CREDENTIALS environment variable
    2. service_account.json in project root

    Returns:
        Path to credentials file or None if not found
    """
    # Check environment variable first
    cred_path = os.getenv("GOOGLE_APPLICATION_CREDENTIALS")
    if cred_path and os.path.exists(cred_path):
        return cred_path

    # Fallback to project root service_account.json
    # Navigate from app/services/ to project root
    project_root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    default_path = os.path.join(project_root, "service_account.json")

    if os.path.exists(default_path):
        return default_path

    return None


def _get_credentials_from_json_env():
    """
    Load Google service account credentials from JSON environment variable.

    This is useful for deployment environments (e.g., Railway) where you can't
    store files but can store the JSON content in an environment variable.

    Returns:
        Credentials object or None if GOOGLE_SERVICE_ACCOUNT_JSON is not set
    """
    json_content = os.getenv("GOOGLE_SERVICE_ACCOUNT_JSON")
    if not json_content:
        return None

    try:
        service_account_info = json.loads(json_content)
        credentials = service_account.Credentials.from_service_account_info(
            service_account_info, scopes=SCOPES
        )
        print("Loaded Google credentials from GOOGLE_SERVICE_ACCOUNT_JSON env var")
        return credentials
    except json.JSONDecodeError as e:
        print(f"Warning: Failed to parse GOOGLE_SERVICE_ACCOUNT_JSON: {e}")
        return None
    except Exception as e:
        print(f"Warning: Failed to load credentials from JSON env var: {e}")
        return None


def _get_credentials():
    """
    Load Google service account credentials.

    Priority:
    1. GOOGLE_SERVICE_ACCOUNT_JSON environment variable (JSON content)
    2. GOOGLE_APPLICATION_CREDENTIALS environment variable (file path)
    3. service_account.json in project root

    Returns:
        Credentials object or None if credentials cannot be loaded
    """
    # Priority 1: JSON content from environment variable
    credentials = _get_credentials_from_json_env()
    if credentials:
        return credentials

    # Priority 2 & 3: File-based credentials
    cred_path = _get_credentials_path()
    if not cred_path:
        print("Warning: No Google service account credentials found")
        return None

    try:
        credentials = service_account.Credentials.from_service_account_file(
            cred_path, scopes=SCOPES
        )
        return credentials
    except Exception as e:
        print(f"Warning: Failed to load Google credentials: {e}")
        return None


def notify_google_indexing(url: str) -> bool:
    """
    Send URL_UPDATED notification to Google Indexing API.

    This function notifies Google that a URL has been updated and should be
    re-crawled and re-indexed. It is designed to fail silently - any errors
    are logged but will NOT crash the main program.

    Args:
        url: The full URL to notify Google about (e.g., https://example.com/page)

    Returns:
        True if notification was successful, False otherwise

    Note:
        - The service account must have Indexing API permission in Google Search Console
        - This function NEVER raises exceptions - all errors are caught and logged
    """
    # Check if Google API libraries are available
    if not GOOGLE_API_AVAILABLE:
        print("Warning: Google API libraries not available, skipping indexing notification")
        return False

    try:
        # Load credentials
        credentials = _get_credentials()
        if not credentials:
            return False

        # Build the Indexing API service
        service = build("indexing", "v3", credentials=credentials)

        # Prepare the request body
        body = {
            "url": url,
            "type": "URL_UPDATED"
        }

        # Send the notification
        response = service.urlNotifications().publish(body=body).execute()

        # Log success
        notification_time = response.get("urlNotificationMetadata", {}).get("latestUpdate", {}).get("notifyTime", "unknown")
        print(f"Google Indexing success for {url} - notifyTime: {notification_time}")

        return True

    except FileNotFoundError as e:
        print(f"Google Indexing: Credentials file not found - {e}")
    except HttpError as e:
        print(f"Google Indexing HTTP error: {e.resp.status} - {e.content.decode('utf-8', errors='ignore')}")
    except Exception as e:
        print(f"Google Indexing failed silently: {type(e).__name__}: {e}")

    return False


def notify_url_deleted(url: str) -> bool:
    """
    Send URL_DELETED notification to Google Indexing API.

    This function notifies Google that a URL has been removed and should be
    de-indexed. Like notify_google_indexing, it fails silently.

    Args:
        url: The full URL to notify Google about removal

    Returns:
        True if notification was successful, False otherwise
    """
    if not GOOGLE_API_AVAILABLE:
        print("Warning: Google API libraries not available, skipping deletion notification")
        return False

    try:
        credentials = _get_credentials()
        if not credentials:
            return False

        service = build("indexing", "v3", credentials=credentials)

        body = {
            "url": url,
            "type": "URL_DELETED"
        }

        response = service.urlNotifications().publish(body=body).execute()
        print(f"Google Indexing (DELETE) success for {url}")

        return True

    except Exception as e:
        print(f"Google Indexing (DELETE) failed silently: {type(e).__name__}: {e}")

    return False
