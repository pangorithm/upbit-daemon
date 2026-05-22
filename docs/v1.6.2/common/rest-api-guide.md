# REST API 사용 및 에러 안내

업비트 REST API 사용을 위한 요청, 인증, 에러 및 gzip 지원 관련 안내입니다.

## Endpoint

```
https://api.upbit.com/v1
```

## TLS

업비트 Open API는 **TLS 1.2 이상** 버전만 지원합니다. TLS 1.3 권장.

## Content Type

- `application/json; charset=utf-8` 지원
- POST 요청은 반드시 JSON 형식으로 본문 전송해야 함
- Form 방식 요청은 2022년 3월 1일부로 지원 종료

## 인증

Exchange API 요청 시 [인증](./auth.md) 가이드의 JWT 토큰을 `Authorization` 헤더에 포함:

```
Authorization: Bearer eyJhb...d8sTw
```

## 요청 수 제한

[요청 수 제한(Rate Limits)](./rate-limits.md) 문서 참조.

## 응답 상태 코드 및 에러

| HTTP Status Code | 관련 에러 코드 | 발생 이유 | 해결 방법 |
|-----------------|---------------|----------|----------|
| 200 OK | - | 정상 응답 | - |
| 201 Created | - | 요청으로 인한 생성 완료 | - |
| 400 Bad Request | create_ask_error, create_bid_error | 주문 요청 정보가 올바르지 않음 | 주문 생성 문서 참조 |
| 400 Bad Request | insufficient_funds_ask, insufficient_funds_bid | 매수/매도 가능 잔고가 부족 | 잔고 확인 |
| 400 Bad Request | under_min_total_ask, under_min_total_bid | 최소 주문 금액 미달 | 페어별 최소 주문 금액 확인 |
| 400 Bad Request | withdraw_address_not_registered | 허용되지 않은 출금 주소 | 등록된 출금 주소 확인 |
| 400 Bad Request | validation_error | 잘못된 API 요청 | 필수 파라미터 누락 확인 |
| 401 Unauthorized | invalid_query_payload | JWT 페이로드가 올바르지 않음 | 서명 생성 확인 |
| 401 Unauthorized | jwt_verification | JWT 검증에 실패 | 토큰 생성 및 서명 점검 |
| 401 Unauthorized | expired_access_key | API 키가 만료 | 새 키 발급 |
| 401 Unauthorized | nonce_used | 이미 사용된 nonce 값 | 매 요청마다 새로운 nonce 사용 |
| 401 Unauthorized | no_authorization_ip | 등록되지 않은 IP | API 키 발급 시 등록한 IP 환경 확인 |
| 401 Unauthorized | no_authorization_token | 인증 토큰 누락 | 인증 헤더 포함 확인 |
| 401 Unauthorized | out_of_scope | 지원 범위를 벗어난 기능 | API 키 권한 확인 |
| 404 Not Found | - | 존재하지 않는 데이터 | 요청 항목 존재 여부 확인 |
| 418 I'm a teapot | - | 과도한 요청으로 거부 | 차단 시간 이후 재시도 |
| 429 Too Many Requests | - | 요청 제한 초과 | API 호출 한도 확인 |
| 500 Internal Server Error | - | 서버 내부 오류 | 서비스 점검 또는 시스템 오류 |

### 에러 응답 형식

**Quotation API Error Response**
```json
{
  "error": {
    "name": 400,
    "message": "ERROR_MESSAGE"
  }
}
```

**Exchange API Error Response**
```json
{
  "error": {
    "name": "ERROR_CODE",
    "message": "ERROR_MESSAGE"
  }
}
```

## 인코딩

GET/DELETE API 시 쿼리 파라미터는 URL 인코딩해야 합니다. 단, `[]`를 포함한 배열 형식 파라미터의 `[`, `]` 문자는 인코딩 대상에서 제외.

## gzip 응답 지원

시세(Quotation) API만 gzip 응답 지원:

```
Accept-Encoding: gzip
```

## API Reference 예제 코드 안내

각 API Reference 우측 상단에서 Shell(cURL), Python, Java(AsyncHttp/java.net.http/OkHttp/Unirest), Node.js(Axios/fetch/https)의 예제 코드를 제공합니다.
