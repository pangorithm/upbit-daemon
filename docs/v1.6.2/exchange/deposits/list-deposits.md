# 입금 목록 조회

최신 입금 목록을 조회합니다.

조회 조건을 설정하여 해당 조건을 만족하는 입금 목록만 조회할 수 있습니다. 통화, 입금 진행 상태, UUID 목록 또는 TXID 목록을 필터 파라미터로 사용할 수 있습니다. 조건을 별도로 지정하지 않는 경우 최근 100개 출금 이력이 반환됩니다

**Revision History**

| 반영 버전 | 반영 일자 | 변경 사항 |
|-----------|-----------|-----------|
| - | 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |
| - | 2020-05-29 | `transaction_type` 필드 추가 |
| - | 2019-07-04 | `state`, `uuid`, `txid` 파라미터 신규 지원 |

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
    "/deposits": {
      "get": {
        "operationId": "list-deposits",
        "summary": "입금 목록 조회",
        "tags": ["입금(Deposit)"],
        "parameters": [
          {
            "name": "currency",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string",
              "description": "조회하고자 하는 통화 코드. \n통화 코드로 조회 대상을 한정하기 위한 필터 파라미터입니다.\n미입력시 최신 입금 내역이 조회됩니다.",
              "example": "KRW"
            }
          },
          {
            "name": "state",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string",
              "enum": ["PROCESSING", "ACCEPTED", "CANCELLED", "REJECTED", "TRAVEL_RULE_SUSPECTED", "REFUNDING", "REFUNDED"],
              "description": "조회하고자 하는 입금 처리 상태.\n입금 처리 상태로 조회 대상을 한정하기 위한 필터 파라미터입니다. 지정한 상태의 입금 정보만 응답으로 반환됩니다.\n\n사용 가능한 값은 다음과 같습니다.\n* `PROCESSING`: 진행중\n* `ACCEPTED`: 완료\n* `CANCELLED`: 취소됨\n* `REJECTED`: 거절됨\n* `TRAVEL_RULE_SUSPECTED`: 트래블룰 추가 인증 대기중\n* `REFUNDING`: 반환 절차 진행중\n* `REFUNDED`: 반환 완료"
            }
          },
          {
            "name": "uuids[]",
            "in": "query",
            "required": false,
            "schema": {
              "type": "array",
              "description": "조회하고자 하는 유일식별자(UUID) 목록.\n지정한 UUID에 해당하는 입출금 정보만 반환됩니다.\n\n[예시] uuids[]=uuid1&uuids[]=uuid2",
              "items": {"type": "string", "example": "9ca023a5-851b-4fec-9f0a-48cd83c2eaae"}
            }
          },
          {
            "name": "txids[]",
            "in": "query",
            "required": false,
            "schema": {
              "type": "array",
              "description": "조회하고자 하는 트랜잭션 ID 목록.\n지정한 txid에 해당하는 입출금 정보만 반환됩니다.\n\n[예시] txids[]=txid1&txids[]=txid2",
              "items": {"type": "string", "example": "98c15999f0bdc4ae0e8a-ed35868bb0c204fe6ec29e4058a3451e-88636d1040f4baddf943274ce37cf9cc"}
            }
          },
          {
            "name": "limit",
            "in": "query",
            "required": false,
            "schema": {
              "type": "integer",
              "description": "요청 개수(default: 100, max: 100)\n요청 당 조회할 주문 개수를 지정합니다. 한번에 최대 100개의 항목을 조회할 수 있으며, 미지정시 기본값은 100입니다.",
              "example": 100,
              "default": 100
            }
          },
          {
            "name": "page",
            "in": "query",
            "required": false,
            "schema": {
              "type": "integer",
              "description": "조회할 페이지 번호.\nPagination을 위한 파라미터로, 조회하고자 하는 페이지를 지정할 수 있습니다. 미지정시 기본값은 1입니다.",
              "example": 1,
              "default": 1
            }
          },
          {
            "name": "order_by",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string",
              "enum": ["asc", "desc"],
              "description": "결과 정렬 방식. \n주문 생성 시각을 기준으로 설정한 방식에 따라 정렬된 주문 목록이 반환됩니다. 사용 가능한 값은 \"desc\"(내림차순, 최신 주문 순) 또는 \"asc\"(오름차순, 오래된 주문 순)입니다. 기본값은 \"desc\"입니다.",
              "example": "desc",
              "default": "desc"
            }
          },
          {
            "name": "from",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string",
              "description": "Pagination을 위한 조회 범위 지정용 커서. \n응답에 포함된 \"uuid\" 값을 이 필드에 입력하여 해당 출금 시각 이후 \"limit\"개의 출금 이력을 이어서 조회할 수 있습니다."
            }
          },
          {
            "name": "to",
            "in": "query",
            "required": false,
            "schema": {
              "type": "string",
              "description": "Pagination을 위한 조회 범위 지정용 커서. \n응답에 포함된 \"uuid\" 값을 이 필드에 입력하여 해당 출금 시각 이전 \"limit\"개의 출금 이력을 조회할 수 있습니다."
            }
          }
        ],
        "responses": {
          "200": {
            "description": "List of deposits",
            "content": {
              "application/json": {
                "schema": {
                  "type": "array",
                  "items": {
                    "type": "object",
                    "properties": {
                      "type": {"type": "string", "description": "입금 종류", "example": "deposit", "default": "deposit"},
                      "uuid": {"type": "string", "description": "입금의 유일식별자(UUID)", "example": "5b871d34-fe38-4025-8f5c-9b22028f85d3"},
                      "currency": {"type": "string", "description": "조회하고자 하는 통화 코드", "example": "BTC"},
                      "net_type": {"type": "string", "nullable": true, "description": "입금 네트워크 유형.", "example": "BTC"},
                      "txid": {"type": "string", "description": "입금 트랜잭션 ID.", "example": "5BC9E3CD3EFCAB866060C5A61A98E6079B4A49BCCFCBF7D220F903F67D1C76C4"},
                      "state": {"type": "string", "enum": ["PROCESSING", "ACCEPTED", "CANCELLED", "REJECTED", "TRAVEL_RULE_SUSPECTED", "REFUNDING", "REFUNDED"], "description": "입금 처리 상태", "example": "ACCEPTED"},
                      "created_at": {"type": "string", "description": "입금 요청 시각 (KST)", "example": "2024-01-01T00:00:00"},
                      "done_at": {"type": "string", "nullable": true, "description": "입금 완료 시각 (KST)", "example": "2024-01-01T00:00:00"},
                      "amount": {"type": "string", "description": "입금하고자 하는 원화의 금액.", "example": "10000"},
                      "fee": {"type": "string", "description": "입금 수수료", "example": 0},
                      "transaction_type": {"type": "string", "enum": ["default", "internal"], "description": "입금 유형", "example": "default", "default": "default"}
                    }
                  }
                },
                "examples": {
                  "Successful Example": {
                    "value": [
                      {
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
                    ]
                  }
                }
              }
            }
          },
          "400": {"description": "error object"},
          "401": {"description": "error object"}
        },
        "description": "최신 입금 목록을 조회합니다."
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
PATH = "/v1/deposits"

ACCESS_KEY = os.environ["UPBIT_OPEN_API_ACCESS_KEY"]
SECRET_KEY = os.environ["UPBIT_OPEN_API_SECRET_KEY"]

params = {
    "currency": "KRW",
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
 --url 'https://api.upbit.com/v1/deposits?currency=KRW' \
 --header 'Authorization: Bearer {JWT_TOKEN}' \
 --header 'accept: application/json'
```
