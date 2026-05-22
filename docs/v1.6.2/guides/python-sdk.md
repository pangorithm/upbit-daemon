# Python SDK

Python 환경에서 Upbit SDK를 사용하여 API를 호출하는 방법을 안내합니다.

## SDK 공식 문서

Upbit Python SDK 관련 문서는 아래에서 확인할 수 있습니다.

* [시작 가이드](https://github.com/upbit-official/upbit-sdk-python/blob/main/README_KR.md) — Upbit Python SDK의 설치 및 기본 사용 방법
* [Upbit SDK API Reference](https://github.com/upbit-official/upbit-sdk-python/blob/main/api.md) — SDK가 지원하는 전체 API 목록
* [SDK 예제 코드](https://github.com/upbit-official/upbit-sdk-python/tree/main/examples) — 예제 코드 및 상세 사용법

## Python 연동 가이드

* 최소 버전: Python 3.9.0+

### 가상 환경 구축 및 SDK 설치

#### 1. 프로젝트 디렉토리 생성

```bash
mkdir upbit_sdk_project
cd upbit_sdk_project
touch upbit_sdk.py
```

#### 2. 가상 환경 생성

```bash
python3 -m venv .venv
```

#### 3. 가상 환경 활성화

* Linux / macOS: `source .venv/bin/activate`
* Windows: `.venv\Scripts\activate`

#### 4. SDK 설치

```bash
pip install upbit-sdk
```

### 클라이언트 인스턴스 설정

Upbit SDK는 인증 정보와 환경 설정을 포함하는 클라이언트 기반으로 동작합니다.

> **권장 방법**: `UPBIT_ACCESS_KEY`, `UPBIT_SECRET_KEY` 환경변수 사용. 소스코드에 API Key를 직접 입력하는 것은 보안상 권장되지 않습니다.

```python
import os
from upbit import Upbit

client = Upbit(
    access_key=os.environ.get("UPBIT_ACCESS_KEY"),
    secret_key=os.environ.get("UPBIT_SECRET_KEY"),
)
```

### 인스턴스 설정 확인

#### 인증이 필요한 API 호출

```python
import os
from upbit import Upbit

def list_accounts():
    client = Upbit(
        access_key=os.environ.get("UPBIT_ACCESS_KEY"),
        secret_key=os.environ.get("UPBIT_SECRET_KEY"),
    )
    accounts = client.accounts.list()
    print(accounts)

if __name__ == "__main__":
    list_accounts()
```

#### 인증 없이 API 호출

```python
from upbit import Upbit

def list_markets():
    client = Upbit()
    markets = client.trading_pairs.list()
    print(markets)

if __name__ == "__main__":
    list_markets()
```

### 응답 예시

#### 인증 API 응답

```json
[
  {
    "currency": "BTC",
    "balance": "0.00050000",
    "locked": "0.00000000",
    "avg_buy_price": "145500000",
    "avg_buy_price_modified": false,
    "unit_currency": "KRW"
  }
]
```

#### 비인증 API 응답

```json
[
  {
    "market": "KRW-BTC",
    "korean_name": "비트코인",
    "english_name": "Bitcoin"
  }
]
```

### 에러 핸들링

```python
import os
import upbit
from upbit import Upbit

client = Upbit(
    access_key=os.environ.get("UPBIT_ACCESS_KEY"),
    secret_key=os.environ.get("UPBIT_SECRET_KEY"),
)

try:
    accounts = client.accounts.list()
    print(accounts)

except upbit.APIConnectionError as e:
    print("서버 연결 실패")
    print(e.__cause__)

except upbit.AuthenticationError:
    print("인증 실패: API Key를 확인하세요.")

except upbit.RateLimitError:
    print("요청 횟수 초과")

except upbit.APIStatusError as e:
    print(f"API 오류 ({e.status_code}): {e.message}")
```

### 에러 타입

| 상태 코드 | 오류 타입 |
| ------ | -------------------------- |
| 400 | `BadRequestError` |
| 401 | `AuthenticationError` |
| 403 | `PermissionDeniedError` |
| 404 | `NotFoundError` |
| 418 | `RateLimitPenaltyError` |
| 422 | `UnprocessableEntityError` |
| 429 | `RateLimitError` |
| >=500 | `InternalServerError` |
| N/A | `APIConnectionError` |
