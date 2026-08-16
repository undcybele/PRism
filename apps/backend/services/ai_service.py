import anthropic
from core.config import settings
from models.schemas import PRAnalysisResponse

client = anthropic.AsyncAnthropic(api_key=settings.anthropic_api_key)


async def analyze_pr(repo: str, pr_number: int) -> PRAnalysisResponse:
    # Placeholder: in real usage, fetch the PR diff first via github_service
    message = await client.messages.create(
        model="claude-sonnet-4-6",
        max_tokens=1024,
        messages=[
            {
                "role": "user",
                "content": f"Analyze PR #{pr_number} in {repo}. Provide a summary and suggestions.",
            }
        ],
    )

    text = message.content[0].text
    return PRAnalysisResponse(
        summary=text,
        suggestions=[],
    )
