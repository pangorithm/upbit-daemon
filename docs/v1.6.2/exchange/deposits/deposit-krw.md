# 원화 입금

입출금 계좌로부터 원화를 지정한 금액만큼 입금합니다.

**POST API에 대한 Form 방식 요청은 2022년 3월 1일부로 지원이 종료되었습니다.**

Form 방식 지원 종료에 따라 Urlencoded Form 방식으로 전송하는 POST 요청에 대한 정상적인 동작을 보장하지 않습니다. <b>반드시 JSON 형식으로 요청 본문(Body)을 전송</b>해주시기 바랍니다.

**Revision History**

| 반영 버전 | 반영 일자 | 변경 사항 |
|-----------|-----------|-----------|
| - | 2022-09-05 | 네이버 인증 수단 추가 |
| - | 2021-01-11 | '원화 입금 요청' 신규 기능 지원 |
| - | 2020-05-29 | `transaction_type` 필드 추가 |

**Rate Limit**
초당 최대 30회 호출할 수 있습니다. 계정단위로 측정되며 [Exchange 기본 그룹] 내에서 요청 가능 횟수를 공유합니다.

**API Key Permission**
<a href="auth">인증</a>이 필요한 API로, [입금하기] 권한이 설정된 API Key를 사용해야 합니다. <br>
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
    "/deposits/krw": {
      "post": {
        "operationId": "deposit-krw",
        "summary": "원화 입금",
        "tags": ["입금(Deposit)"],
        "requestBody": {
          "description": "원화 입금 요청",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "required": ["amount", "two_factor_type"],
                "properties": {
                  "amount": {
                    "type": "string",
                    "description": "입금하고자 하는 원화의 금액.",
                    "example": "10000"
                  },
                  "two_factor_type": {
                    "type": "string",
                    "enum": ["kakao", "naver", "hana"],
                    "description": "원화 입출금 시 사용할 2차 인증 수단.\n사용 가능한 값은 다음과 같습니다.\n\n* `kakao`: 카카오 인증\n* `naver`: 네이버 인증\n* `hana`: 하나인증서 인증",
                    "example": "kakao"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "201": {
            "description": "Object of KRW deposit",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "required": ["type", "uuid", "currency", "txid", "state", "created_at", "done_at", "amount", "fee", "transaction_type"],
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
                      "description": "입금하고자 하는 통화 코드",
                      "example": "KRW"
                    },
                    "net_type": {
                      "type": "string",
                      "nullable": true,
                      "description": "입금 네트워크 유형.\n업비트에서 사용하는 블록체인 네트워크 구분자입니다. 원화(KRW) 입금의 경우 null로 반환됩니다.\n\n[예시] ETH, TRX, SOL",
                      "example": "BTC"
                    },
                    "txid": {
                      "type": "string",
                      "description": "입금 트랜잭션 ID.",
                      "example": "5BC9E3CD3EFCAB866060C5A61A98E6079B4A49BCCFCBF7D220F903F67D1C76C4"
                    },
                    "state": {
                      "type": "string",
                      "enum": ["ACCEPTED", "CANCELLED", "REJECTED", "REFUNDING"],
                      "description": "입금 처리 상태\n  - `ACCEPTED`: 완료\n  - `CANCELLED`: 취소됨\n  - `REJECTED`: 거절됨\n  - `REFUNDING`: 반환 절차 중",
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
                      "uuid": "9f432943-54e0-40b7-825f-b6fec8b42b79",
                      "currency": "KRW",
                      "net_type": null,
                      "txid": "ebe6937b-130e-4066-8ac6-4b0e67f28adc",
                      "state": "ACCEPTED",
                      "created_at": "2025-07-04T15:00:00+09:00",
                      "done_at": null,
                      "amount": "10000",
                      "fee": "0.0",
                      "transaction_type": "default"
                    }
                  }
                }
              }
            }
          },
          "400": {
            "description": "error object",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "error": {
                      "type": "object",
                      "required": ["name", "message"],
                      "properties": {
                        "name": {"type": "string", "description": "에러명"},
                        "message": {"type": "string", "description": "에러 메세지"}
                      }
                    }
                  }
                }
              }
            }
          }
        },
        "description": "입출금 계좌로부터 원화를 지정한 금액만큼 입금합니다."
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
PATH = "/v1/deposits/krw"

ACCESS_KEY = os.environ["UPBIT_OPEN_API_ACCESS_KEY"]
SECRET_KEY = os.environ["UPBIT_OPEN_API_SECRET_KEY"]

params = {
  "amount": "10000",
  "two_factor_type": "naver",
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

res = requests.post(f"{BASE_URL}{PATH}", headers=headers, json=params)
print(res.json())
```

### Node.js (Axios)

```javascript
const axios = require("axios");
const crypto = require("crypto");
const { sign } = require("jsonwebtoken");
const { v4: uuidv4 } = require("uuid");
require("dotenv").config();

const baseURL = "https://api.upbit.com";
const path = "/v1/deposits/krw";

const ACCESS_KEY = process.env.UPBIT_OPEN_API_ACCESS_KEY;
const SECRET_KEY = process.env.UPBIT_OPEN_API_SECRET_KEY;

const params = {
  amount: "10000",
  two_factor_type: "naver",
};

const queryString = new URLSearchParams(params).toString();

const queryHash = crypto
  .createHash("sha512")
  .update(queryString, "utf-8")
  .digest("hex");

const payload = {
  access_key: ACCESS_KEY,
  nonce: uuidv4(),
  query_hash: queryHash,
  query_hash_alg: "SHA512",
};

const jwtToken = sign(payload, SECRET_KEY);

const options = {
  method: "POST",
  url: `${baseURL}${path}`,
  headers: {
    Authorization: `Bearer ${jwtToken}`,
    Accept: "application/json",
  },
  data: params,
};

axios
  .request(options)
  .then((response) => {
    console.log(response.data);
  })
  .catch((error) => {
    console.error(error.response ? error.response.data : error.message);
  });
```

### cURL

```bash
curl --request POST \
    --url 'https://api.upbit.com/v1/deposits/krw' \
    --header 'Authorization: Bearer {JWT_TOKEN}' \
    --header 'Content-Type: application/json' \
    --data '{
  "amount": "10000",
  "two_factor_type": "naver"
}'
```
