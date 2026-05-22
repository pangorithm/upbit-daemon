# Public API 호출

REST API 호출 테스트 가이드를 따라 업비트 API를 호출해보고 성공 응답을 확인할 수 있습니다.

## cURL 이란?

cURL은 HTTP 요청을 CLI 환경에서 전송하고 응답을 확인할 수 있는 경량 커맨드라인 도구입니다.

## cURL을 활용한 REST API 호출 테스트

### 1. 첫 REST API 호출하기: 업비트 지원 페어 조회

```curl
curl -i --request GET \
     --url https://api.upbit.com/v1/market/all \
     --header 'Accept: application/json'
```

응답:

```
HTTP/2 200
content-type: application/json;charset=UTF-8
remaining-req: group=market; min=600; sec=9
limit-by-ip: Yes

[{"market":"BTC-BERA","korean_name":"베라체인","english_name":"Berachain"},...]
```

### 2. 파라미터를 포함한 요청 전송하기: 페어 현재가 조회

```curl
curl -i --request GET \
     --url 'https://api.upbit.com/v1/ticker?markets=KRW-BTC' \
     --header 'accept: application/json'
```

여러 페어 조회:

```curl
curl -i --request GET \
     --url 'https://api.upbit.com/v1/ticker?markets=KRW-BTC,KRW-ETH' \
     --header 'accept: application/json'
```

### 3. 복잡한 파라미터 처리: 캔들 조회

```bash
curl -i --request GET \
     --url 'https://api.upbit.com/v1/candles/days?market=KRW-BTC&to=2025-07-30T00%3A00%3A00%2B09%3A00&count=2' \
     --header 'accept: application/json'
```

> **URL 인코딩**: `:`은 `%3A`로, `+`는 `%2B`로 변환됩니다.

## 마치며

* [API Key 발급 받기](https://docs.upbit.com/kr/docs/api-key) 가이드로 이동하여 API Key 발급
* [Quotation API](https://docs.upbit.com/kr/reference/list-trading-pairs) Reference로 이동하여 다양한 API 명세 확인
