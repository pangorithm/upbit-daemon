# 주문 생성

특정 페어를 매수/매도하기 위한 주문을 생성합니다.

**Endpoint**: `POST https://api.upbit.com/v1/orders`

**Rate Limit**: 초당 최대 8회 (계정 단위, order 그룹)

**API Key Permission**: 주문하기 권한 필요

## 주문 유형 (ord_type)

### 지정가 주문 (`limit`)
사용자 지정 단가보다 유리한 가격에 체결. 상한/하한 통제 가능.

| 파라미터 | 필수 | 설명 |
|---------|------|------|
| `market` | 필수 | 페어 코드 (`KRW-BTC`) |
| `side` | 필수 | `bid`(매수) / `ask`(매도) |
| `ord_type` | 필수 | `limit` |
| `volume` | 필수 | 주문 수량 |
| `price` | 필수 | 주문 단가 (호가 자산 기준) |
| `time_in_force` | 선택 | `ioc`, `fok`, `post_only` |
| `smp_type` | 선택 | `cancel_maker`, `cancel_taker`, `reduce` |
| `identifier` | 선택 | 클라이언트 지정 주문 ID |

### 시장가 매수 주문 (`price`)
현재 시장에서 가장 유리한 가격으로 즉시 매수.

| 파라미터 | 필수 | 설명 |
|---------|------|------|
| `market` | 필수 | 페어 코드 |
| `side` | 필수 | `bid` |
| `ord_type` | 필수 | `price` |
| `price` | 필수 | 매수 총액 (호가 자산 기준) |
| `smp_type` | 선택 | `cancel_maker`, `cancel_taker`, `reduce` |
| `identifier` | 선택 | 클라이언트 지정 주문 ID |

### 시장가 매도 주문 (`market`)
현재 시장에서 가장 유리한 가격으로 즉시 매도.

| 파라미터 | 필수 | 설명 |
|---------|------|------|
| `market` | 필수 | 페어 코드 |
| `side` | 필수 | `ask` |
| `ord_type` | 필수 | `market` |
| `volume` | 필수 | 매도 수량 |
| `smp_type` | 선택 | `cancel_maker`, `cancel_taker`, `reduce` |
| `identifier` | 선택 | 클라이언트 지정 주문 ID |

### 최유리 지정가 주문 (`best`)
현재 시장에서 가장 유리한 상대 호가로 주문.

| 파라미터 | 필수 | 설명 |
|---------|------|------|
| `market` | 필수 | 페어 코드 |
| `side` | 필수 | `bid`(매수) / `ask`(매도) |
| `ord_type` | 필수 | `best` |
| `volume` | 매도 필수 / 매수 선택 | 매도 시 수량, 매수 시 총액 |
| `price` | 매수 필수 / 매도 선택 | 매수 시 총액, 매도 시 수량 |
| `time_in_force` | **최유리 주문 필수** | `ioc`, `fok` |
| `smp_type` | 선택 | `cancel_maker`, `cancel_taker`, `reduce` |
| `identifier` | 선택 | 클라이언트 지정 주문 ID |

## 주문 체결 조건 (time_in_force)

| 옵션 | 값 | 설명 |
|-----|-----|------|
| **IOC** | `ioc` | 즉시 체결 가능 수량만 부분 체결, 잔여 수량 취소. 지정가/최유리 지정가 전용 |
| **FOK** | `fok` | 전량 체결 가능할 때만 실행, 아니면 전량 취소. 지정가/최유리 지정가 전용 |
| **Post Only** | `post_only` | 메이커 주문으로만 생성. 지정가 주문 전용. SMP와 함께 사용 불가 |

## 자전거래 체결 방지 (SMP)

| 옵션 | 값 | 설명 |
|-----|-----|------|
| 메이커 주문 취소 | `cancel_maker` | 기존 주문 취소로 체결 방지 |
| 테이커 주문 취소 | `cancel_taker` | 신규 주문 취소로 체결 방지 |
| 주문 수량 조정 | `reduce` | 기존+신규 주문 수량 줄여 체결 방지 |

## 체결 대기 중 자산 잠금

주문 생성 시 호가 자산/기준 자산이 즉시 잠금(locked) 상태로 전환. 다음 조건 해제 시까지:
- 주문 전량 체결
- 사용자 요청으로 주문 취소
- `time_in_force` 조건으로 주문 만료

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `uuid` | string | 주문 유일 식별자 |
| `side` | string | `ask` / `bid` |
| `ord_type` | string | 주문 유형 |
| `price` | string | 주문 단가/총액 |
| `state` | string | `wait`(체결 대기) / `watch`(예약 대기) / `done`(체결 완료) / `cancel`(취소) |
| `created_at` | string | 주문 생성 시각 (KST) |
| `volume` | string | 주문 요청 수량 |
| `remaining_volume` | string | 체결 후 남은 양 |
| `executed_volume` | string | 체결된 양 |
| `reserved_fee` | string | 수수료로 예약된 비용 |
| `remaining_fee` | string | 남은 수수료 |
| `paid_fee` | string | 사용된 수수료 |
| `locked` | string | 거래에 사용 중인 비용 |
| `trades_count` | integer | 체결 건수 |
| `time_in_force` | string | 체결 옵션 |
| `identifier` | string | 클라이언트 지정 ID |
| `smp_type` | string | SMP 모드 |
| `prevented_volume` | string | SMP로 취소된 수량 |
| `prevented_locked` | string | SMP로 해제된 자산 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-07-07 | Post Only 주문 옵션 신규 지원 |
| 2025-07-02 | SMP 기능 신규 지원 (`smp_type`, `prevented_volume`, `prevented_locked`) |
| 2024-12-04 | `identifier` 필드 신규 지원 |
| 2024-04-22 | 최유리지정가 주문 유형, 주문 옵션 추가 |
