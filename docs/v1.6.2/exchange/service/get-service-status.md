# 입출금 서비스 상태 조회

전체 통화에 대해 입출금 서비스 상태를 조회합니다.

**⚠️ 입출금 서비스 상태 조회 API는 실시간 상태 조회를 보장하지 않습니다.**

입출금 서비스 상태 조회 API가 반환하는 입출금 가능 여부는 서비스 상태를 실시간으로 반영하지 않으며 반영은 수 분 정도 지연될 수 있습니다. <b>따라서 거래 전략 용도가 아닌 참고 용도로의 사용만을 권장</b>하며, 실제 입금을 수행하기 전에는 반드시 <a href="https://upbit.com/service_center/notice">업비트 공지사항</a> 및 <a href="https://upbit.com/service_center/wallet_status">실시간 입출금 현황</a> 페이지를 참고해 주시기를 바랍니다.

**네트워크 타입("net_type")과 네트워크 이름("network_name")**

네트워크 타입("net_type")은 디지털 자산 입출금시 실제 자산이 이동되는 블록체인 네트워크(대상 체인)를 지정하기 위한 식별자 필드(예: BTC)입니다. 디지털 자산 출금 시 필수 파라미터로, 정상적인 입출금 진행을 위해 정확한 식별자 값을 사용해야 합니다.
디지털 자산 출금 API 호출 시 사전에 출금 허용 주소 목록 조회 API를 호출한 뒤 응답으로부터 정확한 네트워크 타입 값을 참조하여 사용하시기 바랍니다.

네트워크 이름("network_name")은 블록체인 네트워크의 전체 이름(예: Bitcoin)을 나타내는 필드로서, 사람이 인식할 수 있는 정보이며 식별자로 사용할 수 없습니다. 서비스 UI 등에서 블록체인 네트워크를 표현하는 용도로 사용할 수 있습니다.

**Revision History**

| 반영 버전 | 반영 일자 | 변경 사항 |
|-----------|-----------|-----------|
| - | 2023-11-22 | 네트워크 명(network_name) 필드 추가 |
| - | 2023-11-22 | 네트워크 타입(net_type) 필드 추가 |
| - | - | '입출금 서비스 상태 조회' 기능 신규 지원 |

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
    "/status/wallet": {
      "get": {
        "operationId": "get-service-status",
        "summary": "입출금 서비스 상태 조회",
        "tags": ["서비스 정보(Service)"],
        "responses": {
          "200": {
            "description": "List of service status",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "required": ["currency", "wallet_state", "block_elapsed_minutes", "net_type", "network_name"],
                    "properties": {
                      "currency": {
                        "type": "string",
                        "description": "조회하고자 하는 통화 코드",
                        "example": "BTC"
                      },
                      "wallet_state": {
                        "type": "string",
                        "enum": ["working", "withdraw_only", "deposit_only", "paused", "unsupported"],
                        "description": "입출금 상태.\n입출금 상태를 나타내는 필드입니다.\n  - `working` : 입출금 가능\n  - `withdraw_only` : 출금만 가능\n  - `deposit_only` : 입금만 가능\n  - `paused` : 입출금 중단\n  - `unsupported` : 입출금 미지원",
                        "example": "working"
                      },
                      "block_state": {
                        "type": "string",
                        "enum": ["normal", "delayed", "inactive"],
                        "description": "블록체인 네트워크의 상태.\n지갑 또는 거래소 상태에 따라 null로 반환 될 수 있습니다.\n  - `normal`: 정상\n  - `delayed`: 지연\n  - `inactive`: 비활성",
                        "example": "normal"
                      },
                      "block_height": {
                        "type": "integer",
                        "description": "현재 확인된 블록의 높이. \n지갑 또는 거래소 상태에 따라 null로 반환 될 수 있습니다.",
                        "example": 902656
                      },
                      "block_updated_at": {
                        "type": "string",
                        "description": "마지막으로 블록 높이가 갱신된 시각 (UTC).\n지갑 또는 거래소 상태에 따라 null로 반환 될 수 있습니다.\n\n[형식] yyyy-MM-dd'T'HH:mm:sss+00:00",
                        "example": "2024-01-01T00:00:000+09:00"
                      },
                      "block_elapsed_minutes": {
                        "type": "integer",
                        "description": "마지막 블록 업데이트 이후 현재까지 경과한 시간(분).\n지갑 또는 거래소 상태에 따라 null로 반환 될 수 있습니다.",
                        "example": 31
                      },
                      "net_type": {
                        "type": "string",
                        "description": "입출금 네트워크 유형.\n업비트에서 사용하는 블록체인 네트워크 구분자입니다.\n\n[예시] \"ETH\", \"TRX\", \"SOL\"",
                        "example": "BTC"
                      },
                      "network_name": {
                        "type": "string",
                        "description": "입출금 네트워크 이름.\n업비트에서 사용자에게 표시되는 블록체인 네트워크 이름입니다.\n\n[예시] \"Ethereum\", \"Bitcoin\", \"Tron\", \"Solana\"",
                        "example": "Bitcoin"
                      }
                    }
                  }
                },
                "examples": {
                  "Successful Example": {
                    "value": [
                      {
                        "currency": "BTC",
                        "wallet_state": "working",
                        "block_state": "normal",
                        "block_height": 903942,
                        "block_updated_at": "2025-07-04T08:02:05.526+00:00",
                        "block_elapsed_minutes": 6,
                        "net_type": "BTC",
                        "network_name": "Bitcoin"
                      },
                      {
                        "currency": "ETH",
                        "wallet_state": "working",
                        "block_state": "normal",
                        "block_height": 22844550,
                        "block_updated_at": "2025-07-04T08:06:44.375+00:00",
                        "block_elapsed_minutes": 2,
                        "net_type": "ETH",
                        "network_name": "Ethereum"
                      },
                      {
                        "currency": "XRP",
                        "wallet_state": "working",
                        "block_state": "normal",
                        "block_height": 97241570,
                        "block_updated_at": "2025-07-04T08:06:53.213+00:00",
                        "block_elapsed_minutes": 2,
                        "net_type": "XRP",
                        "network_name": "XRP Ledger"
                      },
                      {
                        "currency": "USDT",
                        "wallet_state": "working",
                        "block_state": "normal",
                        "block_height": 22844550,
                        "block_updated_at": "2025-07-04T08:06:44.375+00:00",
                        "block_elapsed_minutes": 2,
                        "net_type": "USDT",
                        "network_name": "Tether"
                      },
                      {
                        "currency": "ADA",
                        "wallet_state": "working",
                        "block_state": "normal",
                        "block_height": 12080440,
                        "block_updated_at": "2025-07-04T08:05:48.182+00:00",
                        "block_elapsed_minutes": 3,
                        "net_type": "ADA",
                        "network_name": "Cardano"
                      },
                      {
                        "currency": "TRX",
                        "wallet_state": "working",
                        "block_state": "normal",
                        "block_height": 73652750,
                        "block_updated_at": "2025-07-04T08:06:36.573+00:00",
                        "block_elapsed_minutes": 2,
                        "net_type": "TRX",
                        "network_name": "Tron"
                      }
                    ]
                  }
                }
              }
            }
          }
        },
        "description": "전체 통화에 대해 입출금 서비스 상태를 조회합니다."
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
PATH = "/v1/status/wallet"

ACCESS_KEY = os.environ["UPBIT_OPEN_API_ACCESS_KEY"]
SECRET_KEY = os.environ["UPBIT_OPEN_API_SECRET_KEY"]

payload = {
  "access_key": ACCESS_KEY,
  "nonce": str(uuid.uuid4()),
}

jwt_token = jwt.encode(payload, SECRET_KEY, algorithm="HS256")

res = requests.get(f"{BASE_URL}{PATH}", headers={"Authorization": f"Bearer {jwt_token}", "Accept": "application/json"})
print(res.json())
```

### cURL

```bash
curl --request GET \
    --url 'https://api.upbit.com/v1/status/wallet' \
    --header 'Authorization: Bearer {JWT_TOKEN}' \
    --header 'Accept: application/json'
```
