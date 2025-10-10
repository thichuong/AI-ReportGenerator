"""
APIRouter cho các tính năng về thị trường
"""
import json
import os
from fastapi import APIRouter, HTTPException
from fastapi.responses import JSONResponse
import redis.asyncio as redis

router = APIRouter()

async def fetch_and_store_market_indices():
    """Lấy dữ liệu chỉ số thị trường và lưu vào Redis"""
    try:
        from app.services.vnstock_service import (
            get_vnindex, get_vn30, get_hnx_index, get_hnx30, get_upcom_index
        )

        data = {}
        for name, fn in [
            ('vnindex', get_vnindex),
            ('vn30', get_vn30),
            ('hnxindex', get_hnx_index),
            ('hnx30', get_hnx30),
            ('upcomindex', get_upcom_index),
        ]:
            val, err = fn()
            data[name] = {'value': val, 'error': err}

        response_data = {'success': True, 'indices': data}

        # Store in Redis
        redis_url = os.getenv('REDIS_URL')
        if not redis_url:
            raise ValueError("REDIS_URL environment variable not set")
        r = redis.Redis.from_url(redis_url)
        await r.set('market_indices', json.dumps(response_data), ex=300)  # expire in 5 minutes

        return response_data
    except Exception as e:
        raise e

@router.get('/market/indices')
async def get_market_indices():
    """Trả về các chỉ số thị trường chính: VNINDEX, VN30, HNXINDEX, HNX30, UPCOMINDEX"""
    try:
        response_data = await fetch_and_store_market_indices()
        return JSONResponse(response_data)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
