"""
Service to fetch Vietnam market indices using vnstock package.

Provides simple helper functions to retrieve latest real-time index values for:
 - VN-Index
 - VN30
 - HNX-Index
 - HNX30
 - UPCoM-Index

Each function returns a tuple: (data_dict, error_message)
Data is fetched with 1-minute interval for near real-time values.
"""
from typing import Tuple, Dict, Optional
import logging
import datetime

import numpy as np
import pandas as pd

try:
    # Use the VCI explorer Quote class for index data
    from vnstock.explorer.vci.quote import Quote as VCIQuote
except Exception:
    VCIQuote = None

logger = logging.getLogger(__name__)


def _ensure_vnstock() -> Optional[str]:
    if VCIQuote is None:
        return "vnstock package is not installed or VCI Quote adapter unavailable"
    return None


def _get_start_date(days_back: int = 7) -> str:
    """Get start date string for vnstock queries."""
    return (datetime.datetime.now() - datetime.timedelta(days=days_back)).strftime('%Y-%m-%d')


def _index_to_dict(idx) -> Dict:
    # Normalize vnstock objects to a simple dict
    try:
        # idx is expected to be a pandas Series or object with attributes
        def _to_native(v):
            # pandas Timestamp -> ISO string
            if isinstance(v, pd.Timestamp):
                if pd.isna(v):
                    return None
                # keep date/time precision
                return v.to_pydatetime().isoformat()
            # pandas NaT
            if v is pd.NaT:
                return None
            # numpy scalar -> python native
            if isinstance(v, np.generic):
                try:
                    return v.item()
                except Exception:
                    return float(v)
            # datetime
            if isinstance(v, datetime.datetime):
                return v.isoformat()
            if isinstance(v, datetime.date):
                return v.isoformat()
            # pandas types like numpy arrays or others -> convert where sensible
            if isinstance(v, (list, dict, str, int, float, bool, type(None))):
                return v
            try:
                # fallback: try to convert to python builtin via item() or str
                if hasattr(v, 'item'):
                    return v.item()
            except Exception:
                pass
            try:
                return str(v)
            except Exception:
                return None

        code = getattr(idx, "symbol", None) or getattr(idx, "code", None) or getattr(idx, "name", None)
        date = getattr(idx, "time", None) or getattr(idx, "date", None)
        open_v = getattr(idx, "open", None)
        high_v = getattr(idx, "high", None)
        low_v = getattr(idx, "low", None)
        close_v = getattr(idx, "close", None) or getattr(idx, "last", None)
        volume_v = getattr(idx, "volume", None)

        return {
            "code": _to_native(code),
            "date": _to_native(date),
            "open": _to_native(open_v),
            "high": _to_native(high_v),
            "low": _to_native(low_v),
            "close": _to_native(close_v),
            "volume": _to_native(volume_v),
        }
    except Exception as e:
        logger.debug("Error converting index object to dict: %s", e)
        return {"error": str(e)}


def get_vnindex() -> Tuple[Optional[Dict], Optional[str]]:
    """Return latest real-time VN-Index data."""
    err = _ensure_vnstock()
    if err:
        return None, err

    if VCIQuote is None:
        return None, "vnstock explorer VCI Quote is not available"
    try:
        q = VCIQuote('VNINDEX', show_log=False)
        # Get real-time data with 1-minute interval, last 1 data point
        # Use 7 days ago as start date to ensure data availability
        start_date = _get_start_date()
        df = q.history(start=start_date, interval='1m', count_back=1)
        # df.tail(1) may be a DataFrame
        last = df.tail(1).iloc[0]
        return _index_to_dict(last), None
    except Exception as e:
        logger.exception("Failed to fetch VN-Index")
        return None, str(e)


def get_vn30() -> Tuple[Optional[Dict], Optional[str]]:
    """Return latest real-time VN30 data."""
    err = _ensure_vnstock()
    if err:
        return None, err
    if VCIQuote is None:
        return None, "vnstock explorer VCI Quote is not available"
    try:
        q = VCIQuote('VN30', show_log=False)
        # Get real-time data with 1-minute interval, last 1 data point
        # Use 7 days ago as start date to ensure data availability
        start_date = _get_start_date()
        df = q.history(start=start_date, interval='1m', count_back=1)
        last = df.tail(1).iloc[0]
        return _index_to_dict(last), None
    except Exception as e:
        logger.exception("Failed to fetch VN30")
        return None, str(e)


def get_hnx_index() -> Tuple[Optional[Dict], Optional[str]]:
    if VCIQuote is None:
        return None, "vnstock explorer VCI Quote is not available"
    try:
        q = VCIQuote('HNXINDEX', show_log=False)
        # Get real-time data with 1-minute interval, last 1 data point
        # Use 7 days ago as start date to ensure data availability
        start_date = _get_start_date()
        df = q.history(start=start_date, interval='1m', count_back=1)
        last = df.tail(1).iloc[0]
        return _index_to_dict(last), None
    except Exception as e:
        logger.exception("Failed to fetch HNX Index")
        return None, str(e)


def get_hnx30() -> Tuple[Optional[Dict], Optional[str]]:
    if VCIQuote is None:
        return None, "vnstock explorer VCI Quote is not available"
    try:
        q = VCIQuote('HNX30', show_log=False)
        # Get real-time data with 1-minute interval, last 1 data point
        # Use 7 days ago as start date to ensure data availability
        start_date = _get_start_date()
        df = q.history(start=start_date, interval='1m', count_back=1)
        last = df.tail(1).iloc[0]
        return _index_to_dict(last), None
    except Exception as e:
        logger.exception("Failed to fetch HNX30")
        return None, str(e)


def get_upcom_index() -> Tuple[Optional[Dict], Optional[str]]:
    if VCIQuote is None:
        return None, "vnstock explorer VCI Quote is not available"
    try:
        # UPCOM is exposed as HNXUpcomIndex in mapping
        q = VCIQuote('UPCOMINDEX', show_log=False)
        # Get real-time data with 1-minute interval, last 1 data point
        # Use 7 days ago as start date to ensure data availability
        start_date = _get_start_date()
        df = q.history(start=start_date, interval='1m', count_back=1)
        last = df.tail(1).iloc[0]
        return _index_to_dict(last), None
    except Exception as e:
        logger.exception("Failed to fetch UPCOM Index")
        return None, str(e)
