# 자전거래 체결 방지(Self-Match Prevention, SMP)

자전거래 체결 방지 옵션의 개념과 사용 방법을 안내합니다.

## 자전거래 체결 방지 기능이 무엇인가요?

의도치 않게 동일 회원의 주문이 호가창에서 만날 때 주문 체결을 방지하여 **불필요한 수수료 발생을 줄이고**, **보다 효과적이고 안정적으로 관련 규제를 준수**할 수 있도록 지원하는 기능입니다.

## 어떻게 설정하나요?

| 대상 | 파라미터 |
| :---------------------------------------------------------: | :------------: |
| [POST /v1/orders](https://docs.upbit.com/kr/reference/new-order) | `smp_type` |
| [POST /v1/orders/cancel_and_new](https://docs.upbit.com/kr/reference/cancel-and-new-order) | `new_smp_type` |

## 설정가능한 모드는 무엇이 있나요?

**taker 주문의 설정 기준으로 동작합니다.**

* `cancel_taker`: maker 주문을 유지하고, **taker 주문을 취소**하여 자전거래를 방지합니다.
* `cancel_maker`: taker 주문을 유지하고, **maker 주문을 취소**하여 자전거래를 방지합니다.
* `reduce`: 자기주문이 겹치는 수량만큼만 maker, taker **양쪽 주문 수량을 줄여** 자전거래를 방지합니다.
  * 수량이 남아 있으면 주문은 유지됩니다.

## 자전거래 체결 방지 설정으로 인해 주문이 취소되었는지 어떻게 아나요?

| 필드 | 설명 |
|------|------|
| smp_type | 적용된 자전거래 체결 방지 모드 (`reduce`, `cancel_maker`, `cancel_taker`) |
| prevented_volume | 자전거래 체결 방지 설정으로 인해 취소된 주문 수량 |
| prevented_locked | (매수 시) 취소된 금액, (매도 시) 취소된 수량 |

웹소켓으로 MyOrder 데이터 구독 시, `state` 필드가 `prevented`로 전송될 경우 자전거래 체결 방지 설정으로 인해 취소된 수량입니다.

## 유의사항

* `smp_type`을 명시하지 않으면 자전거래 체결 방지 기능은 적용되지 않습니다.
* 주문 시점이 아닌 체결 시점으로 자전거래를 체크합니다.
* 시장가, 지정가, 예약가, IOC/FOK, 취소 후 재주문 등 모든 주문 타입과 함께 사용 가능합니다.
* `post_only` 설정 시 함께 사용 할 수 없습니다.
* `prevented_locked`는 매수 주문 시 수수료 포함 금액, 매도 주문 시 수량입니다.

## 예시 응답

```json
{
  "uuid": "53afa136-8882-46e5-8119-614ae10e623b",
  "side": "bid",
  "smp_type": "cancel_maker",
  "prevented_volume": 1.174291929,
  "prevented_locked": 0.001706246173
}
```
