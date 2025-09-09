"""
APIRouter cho các tính năng về thị trường
"""
from fastapi import APIRouter, HTTPException
from fastapi.responses import JSONResponse

router = APIRouter()

@router.get('/market/indices')
async def get_market_indices():
    """Trả về các chỉ số thị trường chính: VNINDEX, VN30, HNXINDEX, HNX30, UPCOMINDEX"""
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

        return JSONResponse({'success': True, 'indices': data})
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
