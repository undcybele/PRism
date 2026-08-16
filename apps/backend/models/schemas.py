from pydantic import BaseModel


class PRAnalysisRequest(BaseModel):
    repo: str          # e.g. "owner/repo"
    pr_number: int


class PRAnalysisResponse(BaseModel):
    summary: str
    suggestions: list[str]
