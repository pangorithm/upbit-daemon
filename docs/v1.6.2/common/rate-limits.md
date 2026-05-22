# 요청 수 제한 (Rate Limits)

업비트 API의 요청 수 제한(Rate Limits) 정책 안내 및 구현 가이드입니다.

## 기본 정책 안내

- 모든 요청 수 제한은 **초(Second)** 단위로 적용됩니다.
- API가 속한 Rate Limit 그룹별 **초당 최대 허용 요청 수**가 정의됩니다. 같은 그룹 API 간 요청 수가 함께 집계됩니다.
- 서비스 상황에 따른 추가 제한이 발생할 수 있습니다.
- **Origin 헤더를 포함한 요청**은 별도 정책 적용: 시세 조회 REST API 및 WebSocket 요청에 대해 **10초당 1회**만 허용.

## 제한 단위

| 기능 분류 | 측정 단위 | 설명 |
|---------|---------|------|
| 시세 조회 REST API (Quotation) | IP 단위 | 동일 IP 주소에서 발생한 요청 간 초당 잔여 요청 횟수가 공유/차감 |
| 거래 및 자산 관리 REST API (Exchange) | 계정 단위 | 동일한 계정으로 발급된 여러 API Key 사용 시 계정 단위로 측정 |
| WebSocket 연결 요청 및 데이터 요청 | 계정 단위 / IP 단위 | 인증 포함 시 계정 단위, 미포함 시 IP 단위 |

## Rate Limit 그룹 정책

| Rate Limit 그룹 | 정책 | 대상 API |
|----------------|------|---------|
| **Quotation / market** | 초당 최대 10회 | 페어 목록 조회 |
| **Quotation / candle** | 초당 최대 10회 | 초/분/일/주/월/연 캔들 조회 |
| **Quotation / trade** | 초당 최대 10회 | 페어 체결 이력 조회 |
| **Quotation / ticker** | 초당 최대 10회 | 페어 단위 현재가 조회, 마켓 단위 현재가 조회 |
| **Quotation / orderbook** | 초당 최대 10회 | 호가 정보 조회, 호가 정책 조회 |
| **Exchange / default** | 초당 최대 30회 | 계정 잔고 조회, 주문 조회/취소, 출금/입금 관련 대부분 API, 서비스 상태 조회, API Key 목록 조회 등 |
| **Exchange / order** | 초당 최대 8회 | 주문 생성, 취소 후 재주문 |
| **Exchange / order-test** | 초당 최대 8회 | 주문 생성 테스트 |
| **Exchange / order-cancel-all** | 2초당 최대 1회 | 주문 일괄 취소 |
| **websocket-connect** | 초당 최대 5회 | WebSocket 연결 요청 |
| **websocket-message** | 초당 최대 5회, 분당 100회 | WebSocket 데이터 요청 메시지 전송 |

## 잔여 요청 수 확인 방법

REST API 응답의 `Remaining-Req` 헤더로 잔여 요청 수 정보가 반환됩니다:

```
Remaining-Req: group=default; min=1800; sec=29
```

- `group`: 해당 요청이 포함된 Rate Limit Group
- `min`: Deprecated된 분 단위 요청 제한 정보 필드 (고정 값, 참조 제외)
- `sec`: 현재 잔여 요청 수. 0일 경우 일정 시간 이후 요청 필요

## 기준 초과 요청에 대한 제한 안내

- 초당 최대 허용 요청 수 초과 시 HTTP 429 Too Many Requests 에러 반환
- 429 에러 후 지속 요청 시 동일 IP 또는 계정 단위 요청이 일시적으로 차단 (418 상태 코드 + 차단 시간 정보)
- 정책을 위반한 과도한 요청 반복 시 차단 시간이 점진적으로 증가
