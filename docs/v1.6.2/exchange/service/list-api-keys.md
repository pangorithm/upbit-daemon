# API Key 목록 조회

계정의 모든 API Key 목록과 각 Key의 만료일자를 조회합니다.

**Revision History**

| 반영 버전 | 반영 일자 | 변경 사항 |
|-----------|-----------|-----------|
| - | - | 'API Key 목록 조회' 기능 신규 지원 |

**Rate Limit**
초당 최대 30회 호출할 수 있습니다. 계정단위로 측정되며 [Exchange 기본 그룹] 내에서 요청 가능 횟수를 공유합니다.

**API Key Permission**
<a href="auth">인증</a>이 필요한 API 입니다. 별도 권한은 필요하지 않습니다.

## OpenAPI definition

```json
{
  "openapi": "3.0.1",
  "info": {
    "title": "EXCHANGE API",
    "version": "1.0.0"
  },
  "servers": [
    {
      "url": "https://api.upbit.com/v1"
    }
  ],
  "paths": {
    "/api_keys": {
      "get": {
        "operationId": "list-api-keys",
        "summary": "API Key 목록 조회",
        "tags": ["서비스 정보(Service)"],
        "responses": {
          "200": {
            "description": "List of api keys",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "required": ["access_key", "expire_at"],
                    "properties": {
                      "access_key": {
                        "type": "string",
                        "description": "API Key의 Access Key",
                        "example": "xxxxxxxxxxxxxxxxxxxxxxxx"
                      },
                      "expire_at": {
                        "type": "string",
                        "description": "해당 Access Key의 Deprecated일시 (KST)\n\n[형식] yyyy-MM-dd'T'HH:mm:ss+09:00",
                        "example": "2026-06-25T11:22:54+09:00"
                      }
                    }
                  }
                },
                "examples": {
                  "Successful Example": {
                    "value": [
                      {
                        "access_key": "abcd134567890231bacbd",
                        "expire_at": "2026-07-01T09:00:00+09:00"
                      }
                    ]
                  }
                }
              }
            }
          }
        },
        "description": "계정의 모든 API Key 목록과 각 Key의 만료일자를 조회합니다."
      }
    }
  }
}
```

## 코드 샘플

### Python

```python
import os
import uuid
import jwt
import requests
from dotenv import load_dotenv

load_dotenv()

BASE_URL = "https://api.upbit.com"
PATH = "/v1/api_keys"

ACCESS_KEY = os.environ["UPBIT_OPEN_API_ACCESS_KEY"]
SECRET_KEY = os.environ["UPBIT_OPEN_API_SECRET_KEY"]

payload = {
  "access_key": ACCESS_KEY,
  "nonce": str(uuid.uuid4()),
}

jwt_token = jwt.encode(payload, SECRET_KEY, algorithm="HS256")

headers = {
  "Authorization": f"Bearer {jwt_token}",
  "Accept": "application/json",
}

res = requests.get(f"{BASE_URL}{PATH}", headers=headers)
print(res.json())
```

### cURL

```bash
curl --request GET \
--url 'https://api.upbit.com/v1/api_keys' \
--header 'Authorization: Bearer {JWT_TOKEN}' \
--header 'accept: application/json'
```
