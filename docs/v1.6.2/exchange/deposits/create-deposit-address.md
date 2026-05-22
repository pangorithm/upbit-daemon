# 입금 주소 생성 요청

개인 지갑 또는 타 거래소 자산을 업비트로 입금 하기 위한 입금 주소 생성을 요청합니다.

### 비동기 방식 주소 생성으로 인한 API 응답 객체 구분

입금 주소 생성은 비동기 방식으로 동작합니다. API 호출 시점의 입금 주소 생성 완료 여부에 따라 아래 두가지 응답을 반환할 수 있습니다.

1. 최초 API 요청 직후 반환되는 응답은 주소 생성 요청의 **접수 성공 여부를 반환**하며, 응답은 `success`, `message` 필드만 반환됩니다. 해당 응답은 API 추가 호출 시 주소 생성이 완료되기 이전까지 반환됩니다.
2. 비동기 방식으로 주소 생성이 완료된 이후 API 호출 시 "currency", "net\_type", "deposit\_address"를 포함하는 **생성된 주소 정보가 반환**됩니다. 해당 정보는 통화당 최초 1회 생성 되며, 이후 생성 요청의 응답으로는 기존에 생성된 주소 정보가 반환됩니다.

일정 시간이 지난 후에도 입금 주소가 정상적으로 생성되지 않는 경우, 시간 간격을 두고 이 API를 다시 호출해주시기 바랍니다.

**POST API에 대한 Form 방식 요청은 2022년 3월 1일부로 지원이 종료되었습니다.**

Form 방식 지원 종료에 따라 Urlencoded Form 방식으로 전송하는 POST 요청에 대한 정상적인 동작을 보장하지 않습니다. <b>반드시 JSON 형식으로 요청 본문(Body)을 전송</b>해주시기 바랍니다.

**Revision History**

| 반영 버전 | 반영 일자 | 변경 사항 |
|-----------|-----------|-----------|
| - | 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |

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
    "/deposits/generate_coin_address": {
      "post": {
        "operationId": "create-deposit-address",
        "summary": "입금 주소 생성 요청",
        "tags": ["입금(Deposit)"],
        "requestBody": {
          "description": "입금 주소 생성 요청",
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "required": ["currency", "net_type"],
                "properties": {
                  "currency": {
                    "type": "string",
                    "description": "입금 주소가 생성된 통화 코드",
                    "example": "BTC"
                  },
                  "net_type": {
                    "type": "string",
                    "description": "네트워크 유형",
                    "example": "BTC"
                  }
                }
              }
            }
          }
        },
        "responses": {
          "200": {
            "description": "Object of created deposit address",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "required": ["currency", "net_type", "deposit_address"],
                  "properties": {
                    "currency": {
                      "type": "string",
                      "description": "입금 주소가 생성된 통화 코드",
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
                      "description": "입금 주소",
                      "example": "3GXAGnqLWpZWiChDU2AsJBaVxpnPiLBaxU"
                    },
                    "secondary_address": {
                      "type": "string",
                      "nullable": true,
                      "description": "2차 출금 주소. \n일부 디지털 자산의 경우 입출금 주소가 Destination Tag, Memo, 또는 Message와 같은 2차 주소를 포함합니다. 디지털 자산을 수신할 거래소의 수신 주소(입금 주소) 정보에 2차 주소가 포함되어있다면 이 필드를 반드시 포함하여 출금을 요청해야 합니다.",
                      "example": null
                    }
                  }
                },
                "examples": {
                  "Successful Example": {
                    "value": {
                      "currency": "BTC",
                      "net_type": "BTC",
                      "deposit_address": "3EusRwybuZUhVDeHL7gh3HSLmbhLcy7NqD",
                      "secondary_address": null
                    }
                  }
                }
              }
            }
          },
          "201": {
            "description": "Object of created deposit address",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "required": ["success", "message"],
                  "properties": {
                    "success": {
                      "type": "boolean",
                      "description": "입금 주소 생성 요청의 성공 여부",
                      "example": true,
                      "default": true
                    },
                    "message": {
                      "type": "string",
                      "description": "입금 주소 생성 요청 결과에 대한 메시지",
                      "example": "BTC 입금 주소를 생성 중 입니다."
                    }
                  }
                },
                "examples": {
                  "Successful Example": {
                    "value": {
                      "success": true,
                      "message": "BTC 입금 주소를 생성 중 입니다."
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
        "description": "개인 지갑 또는 타 거래소 자산을 업비트로 입금 하기 위한 입금 주소 생성을 요청합니다."
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
PATH = "/v1/deposits/generate_coin_address"

ACCESS_KEY = os.environ["UPBIT_OPEN_API_ACCESS_KEY"]
SECRET_KEY = os.environ["UPBIT_OPEN_API_SECRET_KEY"]

params = {
  "currency": "BTC",
  "net_type": "BTC",
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
const path = "/v1/deposits/generate_coin_address";

const ACCESS_KEY = process.env.UPBIT_OPEN_API_ACCESS_KEY;
const SECRET_KEY = process.env.UPBIT_OPEN_API_SECRET_KEY;

const params = {
  currency: "BTC",
  net_type: "BTC",
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

### Java

```java
import com.auth0.jwt.JWT;
import com.auth0.jwt.algorithms.Algorithm;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;
import java.util.UUID;
import java.util.stream.Collectors;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.RequestBody;
import okhttp3.Response;
import com.google.gson.Gson;

public class CreateDepositAddress {
    private static final String BASE_URL = "https://api.upbit.com";
    private static final String PATH = "/v1/deposits/generate_coin_address";

    public static void main(String[] args) throws NoSuchAlgorithmException, IOException {
        String accessKey = System.getenv("UPBIT_OPEN_API_ACCESS_KEY");
        String secretKey = System.getenv("UPBIT_OPEN_API_SECRET_KEY");

        Map<String, String> params = new HashMap<>();
        params.put("currency", "BTC");
        params.put("net_type", "BTC");
        String queryString = params.entrySet().stream()
            .map(e -> e.getKey() + "=" + String.valueOf(e.getValue()))
            .collect(Collectors.joining("&"));

        MessageDigest md = MessageDigest.getInstance("SHA-512");
        md.update(queryString.getBytes(StandardCharsets.UTF_8));
        StringBuilder sb = new StringBuilder();
        for (byte b : md.digest()) {
            sb.append(String.format("%02x", b));
        }
        String queryHash = sb.toString();

        Algorithm algorithm = Algorithm.HMAC512(secretKey.getBytes(StandardCharsets.UTF_8));
        String jwtToken = JWT.create()
            .withClaim("access_key", accessKey)
            .withClaim("nonce", UUID.randomUUID().toString())
            .withClaim("query_hash", queryHash)
            .withClaim("query_hash_alg", "SHA512")
            .sign(algorithm);

        String authHeader = "Bearer " + jwtToken;

        String jsonBody = new Gson().toJson(params);
        OkHttpClient client = new OkHttpClient();
        Request request = new Request.Builder()
            .url(BASE_URL + PATH)
            .post(RequestBody.create(jsonBody, okhttp3.MediaType.parse("application/json; charset=utf-8")))
            .addHeader("Content-Type", "application/json")
            .addHeader("Authorization", authHeader)
            .build();

        try (Response response = client.newCall(request).execute()) {
            System.out.println(response.code());
            System.out.println(Objects.requireNonNull(response.body()).string());
        }
    }
}
```

### cURL

```bash
curl --request POST \
  --url 'https://api.upbit.com/v1/deposits/generate_coin_address' \
  --header 'Authorization: Bearer {JWT_TOKEN}' \
  --header 'Content-Type: application/json' \
  --data '{
"currency": "BTC",
"net_type": "BTC"
}'
```
