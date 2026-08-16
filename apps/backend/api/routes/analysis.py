from fastapi import APIRouter
from models.schemas import PRAnalysisRequest, PRAnalysisResponse
from services import ai_service

router = APIRouter()


@router.post("/analyze/pr", response_model=PRAnalysisResponse)
async def analyze_pr(request: PRAnalysisRequest):
    result = await ai_service.analyze_pr(request.repo, request.pr_number)
    return result
