# Upbit API 이용 준비

Upbit API를 사용하기 위해 필요한 사전 준비 절차를 안내합니다.

## 사용 목적 정의

사용 목적에 따라 API Key 필요 여부와 권한(Scope)이 달라집니다. 필요한 기능에 맞게 최소 권한만 설정하는 것을 권장합니다.

API Key는 Upbit PC Web의 [Open API 관리 페이지](https://upbit.com/mypage/open_api_management)에서 발급할 수 있습니다.

## 사용 목적별 권한 안내

| 사용 목적 | API Key 필요 | 필요 권한 | 안내 |
|-----------|-------------|-----------|------|
| [시세 및 마켓 데이터 조회](https://docs.upbit.com/kr/reference/list-trading-pairs) | 불필요 | - | Public API로 조회 가능 |
| [과거 마켓 데이터 수집](https://www.upbit.com/historical_data/main) | 불필요 | - | Historical Market Data 문서 참고 |
| [자산 조회 및 관리](https://docs.upbit.com/kr/reference/get-balance) | 필요 | 자산조회 | 잔고 및 보유 자산 조회 |
| [주문](https://docs.upbit.com/kr/reference/available-order-information) | 필요 | 주문조회, 주문하기 | 주문 API 사용 |
| [입금](https://docs.upbit.com/kr/reference/available-deposit-information) | 필요 | 입금조회, 입금하기 | 입금 및 조회 가능 |
| [출금](https://docs.upbit.com/kr/reference/available-withdrawal-information) | 필요 | 출금조회, 출금하기 | 디지털 자산 출금 시 허용 주소 등록 필요 |

> ⚠️ 오류 해결
> * **401 Unauthorized(out_of_scope)**: API Key에 필요한 권한이 포함되어 있지 않은 경우 필요합니다.
> * **400 Bad Request(withdraw_address_not_registered)**: 출금허용주소가 등록되지 않은 경우 필요합니다.

## 다음 단계

* [API Key 발급 받기](https://docs.upbit.com/kr/docs/api-key)
* [거래소 지갑 주소 등록](https://docs.upbit.com/kr/docs/open-api-withdraw_access_register)
* [개인지갑 주소 등록](https://docs.upbit.com/kr/docs/open-api-withdraw-private-wallet)
* [개발환경 설정](https://docs.upbit.com/kr/docs/dev-environment)
