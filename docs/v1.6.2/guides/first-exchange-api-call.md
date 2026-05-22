# 인증 API 호출

발급받은 API Key의 Access Key와 Secret Key를 사용하여 인증 토큰을 생성하고 Exchange API를 호출할 수 있습니다.

## 인증 방식 알아보기

[인증](https://docs.upbit.com/kr/reference/auth) 문서에서 업비트의 REST API와 WebSocket에 사용되는 인증 토큰의 구조 및 생성 방법을 참고할 수 있습니다.

## 인증 토큰 생성을 위한 Shell 스크립트

```shell
#!/bin/bash

ACCESS_KEY="YOUR_ACCESS_KEY"
SECRET_KEY="YOUR_SECRET_KEY"

INPUT_CURL=$1

METHOD=$(echo "$INPUT_CURL" | sed -n "s/.*-X \([A-Z]*\).*/\1/p")
URL=$(echo "$INPUT_CURL" | sed -n "s/.*--url '\([^']*\)'.*/\1/p")
BODY=$(echo "$INPUT_CURL" | sed -n "s/.*--data '\([^']*\)'.*/\1/p")

if [[ "$METHOD" == "POST" ]]; then
  QUERY_STRING=$(echo "$BODY" | tr -d '{}' | sed 's/"//g' | tr ',' '&' | tr ':' '=' | sed 's/ //g')
elif [[ "$METHOD" == "GET" || "$METHOD" == "DELETE" ]]; then
  QUERY_STRING=$(echo "$URL" | awk -F '?' '{print $2}')
fi

if [ -z "$QUERY_STRING" ]; then
  QUERY_HASH=""
else
  QUERY_HASH=$(echo -n "$QUERY_STRING" | openssl dgst -sha512 | sed 's/^.* //')
fi

NONCE=$(uuidgen)
HEADER='{"alg":"HS512","typ":"JWT"}'
PAYLOAD=$(jq -n --arg ak "$ACCESS_KEY" --arg n "$NONCE" --arg qh "$QUERY_HASH" --arg alg "SHA512" \
  '{access_key: $ak, nonce: $n, query_hash: $qh, query_hash_alg: $alg}')

HEADER_BASE64=$(echo -n "$HEADER" | openssl base64 -A | tr '+/' '-_' | tr -d '=')
PAYLOAD_BASE64=$(echo -n "$PAYLOAD" | openssl base64 -A | tr '+/' '-_' | tr -d '=')

SIGNATURE=$(echo -n "$HEADER_BASE64.$PAYLOAD_BASE64" | \
  openssl dgst -sha512 -hmac "$SECRET_KEY" -binary | \
  openssl base64 -A | tr '+/' '-_' | tr -d '=')

JWT="$HEADER_BASE64.$PAYLOAD_BASE64.$SIGNATURE"

CLEANED_CURL=$(echo "$INPUT_CURL" | sed "s/-H 'Authorization: Bearer [^']*'//g")

if echo "$CLEANED_CURL" | grep -q "Content-Type"; then
    FINAL_CURL=$(echo "$CLEANED_CURL" | sed "s/curl /curl -H 'Authorization: Bearer $JWT' /")
else
    FINAL_CURL=$(echo "$CLEANED_CURL" | sed "s/curl /curl -H 'Authorization: Bearer $JWT' -H 'Content-Type: application\/json' /")
fi

echo "[+] Signed curl command:"
echo "$FINAL_CURL"
```

### 잔고 조회 하기

```shell
./auth_curl.sh "curl --request GET --url https://api.upbit.com/v1/accounts --header 'accept: application/json'"
```

### 매수 주문 생성하기

```shell
./auth_curl.sh "curl -X POST --url 'https://api.upbit.com/v1/orders' --header 'accept: application/json' --header 'content-type: application/json' --data '{\"market\":\"KRW-BTC\",\"side\":\"bid\",\"volume\":\"0.0001\",\"price\":\"50000000\",\"ord_type\":\"limit\"}'"
```

> ⚠️ 위 cURL 요청 실행시 실제 주문이 생성됩니다.

## 인증 토큰 생성 방식 이해하기

```shell
if [ -z "$QUERY_STRING" ]; then
  QUERY_HASH=""
else
  QUERY_HASH=$(echo -n "$QUERY_STRING" | openssl dgst -sha512 | sed 's/^.* //')
fi

NONCE=$(uuidgen)
HEADER='{"alg":"HS512","typ":"JWT"}'
PAYLOAD=$(jq -n --arg ak "$ACCESS_KEY" --arg n "$NONCE" --arg qh "$QUERY_HASH" --arg alg "SHA512" \
  '{access_key: $ak, nonce: $n, query_hash: $qh, query_hash_alg: $alg}')

HEADER_BASE64=$(echo -n "$HEADER" | openssl base64 -A | tr '+/' '-_' | tr -d '=')
PAYLOAD_BASE64=$(echo -n "$PAYLOAD" | openssl base64 -A | tr '+/' '-_' | tr -d '=')

SIGNATURE=$(echo -n "$HEADER_BASE64.$PAYLOAD_BASE64" | \
  openssl dgst -sha512 -hmac "$SECRET_KEY" -binary | \
  openssl base64 -A | tr '+/' '-_' | tr -d '=')

JWT="$HEADER_BASE64.$PAYLOAD_BASE64.$SIGNATURE"
```

## 마치며

* [개발 환경 설정 가이드](https://docs.upbit.com/kr/docs/dev-environment)
* [REST API 연동 Best Practice](https://docs.upbit.com/kr/docs/rest-api-best-practice)
