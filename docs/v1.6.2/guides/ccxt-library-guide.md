# CCXT 라이브러리 연동 안내

CCXT(CryptoCurrency eXchange Trading Library)는 전 세계 디지털 자산 거래소의 API와 각 거래소에서 지원하는 알고리즘 트레이딩, 전략 백테스팅 등의 기능을 다양한 프로그래밍 언어로 사용할 수 있게 도와주는 오픈 소스 라이브러리입니다.

## CCXT 공식 문서

* **CCXT 업비트 API 안내**: <https://docs.ccxt.com/#/exchanges/upbit>
* **CCXT 소스 코드**: <https://github.com/ccxt/ccxt>
* **CCXT Docs**: <https://docs.ccxt.com/>

## Python 연동 가이드

* 최소 버전: Python 3.7.0+

### 가상 환경 구축 및 CCXT 라이브러리 설치

1. **프로젝트 디렉토리 및 파일 생성**

```shell
mkdir ccxt_project
cd ccxt_project
touch ccxt_upbit.py
```

2. **가상 환경 생성**

```shell
python3 -m venv .venv
```

3. **가상 환경 활성화**

* Linux/MacOS: `source .venv/bin/activate`
* Windows: `.venv\Scripts\activate`

4. **CCXT 라이브러리 다운로드**

```shell
pip install ccxt
```

5. **CCXT 인스턴스 설정**

```python
# ccxt_upbit.py
import ccxt

access_key = "<YOUR_ACCESS_KEY>"
secret_key = "<YOUR_SECRET_KEY>"

client = ccxt.upbit({
    "apiKey": access_key,
    "secret": secret_key
})
```

6. **인스턴스 설정 확인**

```python
# API Key 설정 API 호출
import ccxt

def list_balances():
    client = ccxt.upbit({
        "apiKey": access_key,
        "secret": secret_key,
    })
    balance = client.fetchBalance()
    print(balance)

if __name__ == "__main__":
    list_balances()
```

```python
# API Key 비설정 API 호출
import ccxt

def list_markets(symbols: list[str]):
    client = ccxt.upbit()
    market_list = client.fetchMarkets()
    return [item for item in market_list if item.get('symbol') in symbols]

if __name__ == "__main__":
    symbols = ["BTC/KRW", "ETH/KRW", "SOL/KRW"]
    markets = list_markets(symbols)
    print(markets)
```

### API Key 설정 API 호출 결과

```json
{
  "info": [
    {
      "currency": "BTC",
      "balance": "0.00000104",
      "locked": "0",
      "avg_buy_price": "160210789.42247014",
      "unit_currency": "KRW"
    }
  ],
  "BTC": {
    "free": 1.04e-6,
    "used": 0.0,
    "total": 1.04e-6
  }
}
```

### API Key 비설정 API 호출 결과

```json
[
  {
    "id": "KRW-BTC",
    "lowercaseId": null,
    "symbol": "BTC/KRW",
    "base": "BTC",
    "quote": "KRW",
    "active": true,
    "taker": 0.0005,
    "maker": 0.0005
  }
]
```

## Node.js 연동 가이드

* 최소 버전: Node v7.6+

### NPM을 통한 CCXT 라이브러리 다운로드

1. **프로젝트 디렉토리 및 파일 생성**

```shell
mkdir ccxt_project
cd ccxt_project
touch ccxt.js
```

2. **CCXT 라이브러리 다운로드**

```shell
npm install ccxt
```

3. **CCXT 인스턴스 설정**

```javascript
// ccxt.js
const ccxt = require('ccxt');

const accessKey = "<YOUR_ACCESS_KEY>";
const secretKey = "<YOUR_SECRET_KEY>";

const client = new ccxt.upbit({
    apiKey: accessKey,
    secret: secretKey,
});
```

4. **인스턴스 설정 확인**

```javascript
// API Key 설정 API 호출
const ccxt = require('ccxt');

const accessKey = "<YOUR_ACCESS_KEY>";
const secretKey = "<YOUR_SECRET_KEY>";

const client = new ccxt.upbit({
    apiKey: accessKey,
    secret: secretKey,
});

async function listBalances() {
    const balance = await client.fetchBalance();
    console.log(balance);
}

listBalances();
```

```javascript
// API Key 비설정 API 호출
const ccxt = require('ccxt');

const upbit = new ccxt.upbit();
const symbols = ["BTC/KRW", "ETH/KRW", "SOL/KRW"];

async function listMarkets(symbols) {
    const markets = await upbit.fetchMarkets();
    return markets.filter(market => symbols.includes(market.symbol));
}

listMarkets(symbols).then(markets => {
    console.log(markets);
});
```
