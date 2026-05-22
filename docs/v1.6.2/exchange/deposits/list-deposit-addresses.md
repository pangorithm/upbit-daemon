# 입금 주소 목록 조회

계정의 모든 입금 주소 목록을 조회합니다. 정상적으로 생성된 모든 입금 주소 목록이 반환됩니다.

**Revision History**

| 반영 버전 | 반영 일자 | 변경 사항 |
|-----------|-----------|-----------|
| - | 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |

**Rate Limit**
초당 최대 30회 호출할 수 있습니다. 계정단위로 측정되며 [Exchange 기본 그룹] 내에서 요청 가능 횟수를 공유합니다.

**API Key Permission**
<a href="auth">인증</a>이 필요한 API로, [입금조회] 권한이 설정된 API Key를 사용해야 합니다. <br>
권한 오류(out_of_scope) 오류가 발생한다면, <a href="https://upbit.com/mypage/open_api_management">API Key 관리 메뉴</a>에서 권한 설정을 확인해주세요.

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
    "/deposits/coin_addresses": {
      "get": {
        "operationId": "list-deposit-addresses",
        "summary": "입금 주소 목록 조회",
        "tags": ["입금(Deposit)"],
        "responses": {
          "200": {
            "description": "List of deposit addresses",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "required": ["currency", "net_type"],
                    "properties": {
                      "currency": {
                        "type": "string",
                        "description": "조회하고자 하는 통화 코드",
                        "example": "BTC"
                      },
                      "net_type": {
                        "type": "string",
                        "nullable": true,
                        "description": "입금 네트워크 유형.\n업비트에서 사용하는 블록체인 네트워크 구분자입니다.\n\n[예시] ETH, TRX, SOL",
                        "example": "BTC"
                      },
                      "deposit_address": {
                        "type": "string",
                        "description": "입금 주소. 생성이 완료되지 않은 경우 null로 반환될 수 있습니다.",
                        "example": "3GXAGnqLWpZWiChDU2AsJBaVxpnPiLBaxU"
                      },
                      "secondary_address": {
                        "type": "string",
                        "nullable": true,
                        "description": "2차 출금 주소. \n일부 디지털 자산의 경우 입출금 주소가 Destination Tag, Memo, 또는 Message와 같은 2차 주소를 포함합니다. 디지털 자산을 수신할 거래소의 수신 주소(입금 주소) 정보에 2차 주소가 포함되어있다면 이 필드를 반드시 포함하여 출금을 요청해야 합니다.",
                        "example": null
                      }
                    }
                  }
                },
                "examples": {
                  "Successful Example": {
                    "value": [
                      {
                        "currency": "BTC",
                        "net_type": "BTC",
                        "deposit_address": "3EusRwybuZUhVDeHL7gh3HSLmbhLcy7NqD",
                        "secondary_address": null
                      },
                      {
                        "currency": "ETH",
                        "net_type": "ETH",
                        "deposit_address": "0x0d73e0a482b8cf568976d2e8688f4a899d29301c",
                        "secondary_address": null
                      },
                      {
                        "currency": "XRP",
                        "net_type": "XRP",
                        "deposit_address": "rN9qNpgnBaZwqCg8CvUZRPqCcPPY7wfWep",
                        "secondary_address": "3057887915"
                      }
                    ]
                  }
                }
              }
            }
          }
        },
        "description": "계정의 모든 입금 주소 목록을 조회합니다. 정상적으로 생성된 모든 입금 주소 목록이 반환됩니다."
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
PATH = "/v1/deposits/coin_addresses"

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
  --url 'https://api.upbit.com/v1/deposits/coin_addresses' \
  --header 'Authorization: Bearer {JWT_TOKEN}' \
  --header 'Accept: application/json'
```
