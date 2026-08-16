from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    anthropic_api_key: str = ""
    github_token: str = ""
    port: int = 8000

    class Config:
        env_file = ".env"


settings = Settings()
