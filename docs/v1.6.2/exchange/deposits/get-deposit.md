# 개별 입금 조회

최신 입금 이력을 조회합니다. 특정 입금 정보를 조회하고자 하는 경우 입금의 UUID 또는 트랜잭션 ID(TXID), 통화 코드로 지정할 수 있습니다.

**Revision History**

| 반영 버전 | 반영 일자 | 변경 사항 |
|-----------|-----------|-----------|
| - | 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |
| - | 2020-05-29 | `transaction_type` 필드 추가 |

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
    "/deposit": {
      "get": {
        "operationId": "get-deposit",
        "summary": "개별 입금 조회",
        "tags": ["입금(Deposit)"],
        "parameters": [
          {
            "name": "currency",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string",
              "description": "조회하고자 하는 통화 코드. \n통화 코드로 조회 대상을 한정하기 위한 필터 파라미터입니다.",
              "example": "BTC"
            }
          },
          {
            "name": "uuid",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string",
              "description": "조회하고자 하는 입금의 유일식별자(UUID)\nuuid와 txid를 모두 입력하지 않는 경우 최신 입금 정보가 반환됩니다.",
              "example": "9ca023a5-851b-4fec-9f0a-48cd83c2eaae"
            }
          },
          {
            "name": "txid",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string",
              "description": "조회하고자 하는 입금의 트랜잭션 ID\nuuid와 txid를 모두 입력하지 않는 경우 최신 입금 정보가 반환됩니다.",
              "example": "98c15999f0bdc4ae0e8a-ed35868bb0c204fe6ec29e4058a3451e-88636d1040f4baddf943274ce37cf9cc"
            }
          }
        ],
        "responses": {
          "200": {
            "description": "Object of deposit",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "required": ["type", "uuid", "currency", "net_type", "txid", "state", "created_at", "done_at", "amount", "fee", "transaction_type"],
                  "properties": {
                    "type": {
                      "type": "string",
                      "description": "입금 종류",
                      "example": "deposit",
                      "default": "deposit"
                    },
                    "uuid": {
                      "type": "string",
                      "description": "입금의 유일식별자(UUID)",
                      "example": "5b871d34-fe38-4025-8f5c-9b22028f85d3"
                    },
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
                    "txid": {
                      "type": "string",
                      "description": "입금 트랜잭션 ID.",
                      "example": "5BC9E3CD3EFCAB866060C5A61A98E6079B4A49BCCFCBF7D220F903F67D1C76C4"
                    },
                    "state": {
                      "type": "string",
                      "enum": ["PROCESSING", "ACCEPTED", "CANCELLED", "REJECTED", "TRAVEL_RULE_SUSPECTED", "REFUNDING", "REFUNDED"],
                      "description": "입금 처리 상태\n  - `PROCESSING`: 입금 진행 중 (디지털 자산만 해당)\n  - `ACCEPTED`: 완료\n  - `CANCELLED`: 취소됨\n  - `REJECTED`: 거절됨\n  - `TRAVEL_RULE_SUSPECTED`: 트래블룰 추가 인증 대기중 (디지털 자산만 해당)\n  - `REFUNDING`: 반환 절차 중\n  - `REFUNDED`: 반환됨 (디지털 자산만 해당)",
                      "example": "ACCEPTED"
                    },
                    "created_at": {
                      "type": "string",
                      "description": "입금 요청 시각 (KST)\n\n[형식] yyyy-MM-ddTHH:mm:ss+09:00",
                      "example": "2024-01-01T00:00:00"
                    },
                    "done_at": {
                      "type": "string",
                      "nullable": true,
                      "description": "입금 완료 시각 (KST)\n\n[형식] yyyy-MM-ddTHH:mm:ss+09:00",
                      "example": "2024-01-01T00:00:00"
                    },
                    "amount": {
                      "type": "string",
                      "description": "입금하고자 하는 원화의 금액.",
                      "example": "10000"
                    },
                    "fee": {
                      "type": "string",
                      "description": "입금 수수료",
                      "example": 0
                    },
                    "transaction_type": {
                      "type": "string",
                      "enum": ["default", "internal"],
                      "description": "입금 유형\n  - `default`: 일반 입금\n  - `internal`: 바로 입금 (업비트 계정간 입금)",
                      "example": "default",
                      "default": "default"
                    }
                  }
                },
                "examples": {
                  "Successful Example": {
                    "value": {
                      "type": "deposit",
                      "uuid": "94332e99-3a87-4a35-ad98-28b0c969f830",
                      "currency": "KRW",
                      "net_type": null,
                      "txid": "BKD-2000-12-29-aeked29c05eadac293b4214994",
                      "state": "ACCEPTED",
                      "created_at": "2025-07-04T15:00:00+09:00",
                      "done_at": "2025-07-04T15:00:10+09:00",
                      "amount": "100000.0",
                      "fee": "0.0",
                      "transaction_type": "default"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "error object"
          },
          "401": {
            "description": "error object",
            "content": {
              "application/json": {
                "examples": {
                  "invalid query payload error": {
                    "value": {
                      "error": {
                        "name": "invalid_query_payload",
                        "message": "Jwt의 query를 검증하는데 실패하였습니다."
                      }
                    }
                  }
                }
              }
            }
          },
          "404": {
            "description": "error object",
            "content": {
              "application/json": {
                "examples": {
                  "not found deposit error": {
                    "value": {
                      "error": {
                        "name": "deposit_not_found",
                        "message": "입출금 정보를 찾지 못했습니다."
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "description": "최신 입금 이력을 조회합니다. 특정 입금 정보를 조회하고자 하는 경우 입금의 UUID 또는 트랜잭션 ID(TXID), 통화 코드로 지정할 수 있습니다."
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
import hashlib
import jwt
import requests
from urllib.parse import unquote, urlencode
from dotenv import load_dotenv

load_dotenv()

BASE_URL = "https://api.upbit.com"
PATH = "/v1/deposit"

ACCESS_KEY = os.environ["UPBIT_OPEN_API_ACCESS_KEY"]
SECRET_KEY = os.environ["UPBIT_OPEN_API_SECRET_KEY"]

params = {
  "uuid": "94332e99-3a87-4a35-ad98-28b0c969f830",
}

query_string = unquote(urlencode(params, doseq=True)).encode("utf-8")

m = hashlib.sha512()
m.update(query_string)
query_hash = m.hexdigest()

payload = {
  "access_key": ACCESS_KEY,
  "nonce": str(uuid.uuid4()),
  "query_hash": query_hash,
  "query_hash_alg": "SHA512",
}

jwt_token = jwt.encode(payload, SECRET_KEY, algorithm="HS256")

headers = {
  "Authorization": f"Bearer {jwt_token}",
  "Accept": "application/json",
}

res = requests.get(f"{BASE_URL}{PATH}", headers=headers, params=params)
print(res.json())
```

### cURL

```bash
curl --request GET \
  --url 'https://api.upbit.com/v1/deposit?uuid=94332e99-3a87-4a35-ad98-28b0c969f830' \
  --header 'Authorization: Bearer {JWT_TOKEN}' \
  --header 'Accept: application/json'
```
