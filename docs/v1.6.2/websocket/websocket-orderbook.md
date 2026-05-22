# 호가 (Orderbook)

호가 데이터를 WebSocket으로 수신하기 위한 요청 및 구독 데이터 예시를 제공합니다.

## 호가 모아보기 (level)

원화마켓(KRW)에서만 지원하는 기능으로, 지정한 단위로 ask/bid price와 size를 모아(group) 조회할 수 있습니다. 숫자 형식의 String으로 요청합니다.

[예시]: KRW-BTC 종목에 대해 level=100000으로 요청시 10만원(KRW) 단위로 ask/bid price가 반환되며, 각 금액대 내에 포진된 매수/매도 주문량의 합산이 size로 반환됩니다.

종목별 호가 단위에 따라 지원하는 모아보기 단위가 다릅니다. 지원하는 모아보기 단위 정보는 [마켓별 주문 정책](https://docs.upbit.com/kr/docs/faq-market-policy) 문서 또는 [호가 정책 조회](https://docs.upbit.com/kr/reference/list-orderbook-instruments) API 응답을 참고하여 사용해주시기 바랍니다. 별도로 지정하지 않는 경우 기본 단위인 0으로 지정됩니다. 미지원 단위를 지정하여 요청하는 경우 데이터가 수신되지 않을 수 있으므로 호출 전 지원하는 단위를 반드시 확인해주시기 바랍니다.

## 호가 조회 단위(개수) 지정

조회할 호가 쌍의 개수 단위(unit)를 지정하고자 하는 경우 기본 요청과 같이 `codes` 필드에 조회할 페어 코드를 입력하되, 페어 코드 뒤에 반점(.)과 조회 단위를 명시하여 지정합니다. 지원하는 호가 조회 단위는 1, 5, 15, 30입니다. 별도의 요청이 없는 경우 기본적으로 30개의 호가 쌍(매수/매도)이 반환됩니다.

```
{pair_code}.{unit}
예시: KRW-BTC.15
```

## Request 메세지 형식

호가 데이터 수신을 요청하기 위해서는 WebSocket 연결 이후 아래 구조의 JSON Object를 생성한 뒤 요청 메세지의 Data Type Object로 포함하여 전송해야 합니다. Ticket, Format 필드를 포함한 전체 WebSocket 데이터 요청 메세지 명세는 [WebSocket 사용 안내](https://docs.upbit.com/kr/reference/websocket-guide) 문서를 참고해주세요.

| 필드명 | 타입 | 내용 | 필수 여부 | 기본 값 |
|--------|------|------|-----------|---------|
| type | String | `orderbook` | Required | - |
| codes | List:String | 수신하고자 하는 페어 목록. 반드시 대문자로 요청해야 합니다. | Required | - |
| level | Double | 모아보기 단위 | Optional | - |
| is_only_snapshot | Boolean | 스냅샷 시세만 제공 | Optional | `false` |
| is_only_realtime | Boolean | 실시간 시세만 제공 | Optional | `false` |

### 예시 - DEFAULT

```json
[
  {
    "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
  },
  {
    "type": "orderbook",
    "codes": ["KRW-BTC","KRW-ETH.5"],
    "level": 10000
  },
  {
    "format": "DEFAULT"
  }
]
```

// 또는 각 페어별로 모아보기 단위를 지정하고자 하는 경우

```json
[
    {
        "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
    },
    {
        "type": "orderbook",
        "codes": ["KRW-BTC"],
        "level": 10000
    },
    {
        "type": "orderbook",
        "codes": ["KRW-BTT"],
        "level": 0
    },
    {
        "format": "DEFAULT"
    }
]
```

## 구독 데이터 명세

| 필드명 | 축약형 | 내용 | 타입 | 값 |
|--------|--------|------|------|-----|
| type | ty | 타입 | String | `orderbook` |
| code | cd | 페어 코드 | String | `KRW-BTC` |
| total_ask_size | tas | 호가 매도 총 잔량 | Double | - |
| total_bid_size | tbs | 호가 매수 총 잔량 | Double | - |
| orderbook_units | obu | 호가 | List of Objects | - |
| orderbook_units.ask_price | obu.ap | 매도 호가 | Double | - |
| orderbook_units.bid_price | obu.bp | 매수 호가 | Double | - |
| orderbook_units.ask_size | obu.as | 매도 잔량 | Double | - |
| orderbook_units.bid_size | obu.bs | 매수 잔량 | Double | - |
| timestamp | tms | 타임스탬프 (ms) | Long | - |
| level | lv | 호가 모아보기 단위 (default: 0, 기본 호가단위) | Double | 모아보기 단위 |
| stream_type | st | 스트림 타입 | String | `SNAPSHOT` (스냅샷), `REALTIME` (실시간) |

**※ 호가 모아보기 기능은 원화마켓(KRW)에서만 지원하므로 BTC, USDT 마켓의 경우 0만 존재합니다.**

### 예시 - DEFAULT

```json
{
  "type": "orderbook",
  "code": "KRW-BTC",
  "timestamp": 1746601573804,
  "total_ask_size": 4.79158413,
  "total_bid_size": 2.65609625,
  "orderbook_units": [
    {
      "ask_price": 137002000,
      "bid_price": 137001000,
      "ask_size": 0.10623869,
      "bid_size": 0.03656812
    },
    {
      "ask_price": 137023000,
      "bid_price": 137000000,
      "ask_size": 0.06144079,
      "bid_size": 0.33543284
    },
    {
      "ask_price": 137050000,
      "bid_price": 136999000,
      "ask_size": 0.0055433,
      "bid_size": 0.00104379
    }
  ],
  "stream_type": "SNAPSHOT",
  "level": 0
}
```

### 예시 - SIMPLE_LIST

```json
[
  {
    "ty": "orderbook",
    "cd": "KRW-BTC",
    "tms": 1751855921432,
    "tas": 17.12835169,
    "tbs": 4.81969018,
    "obu": [
      {
        "ap": 148880000,
        "bp": 148830000,
        "as": 0.37765316,
        "bs": 0.34809059
      },
      {
        "ap": 148890000,
        "bp": 148820000,
        "as": 0.64120607,
        "bs": 0.02744065
      },
      {
        "ap": 148900000,
        "bp": 148810000,
        "as": 0.70085443,
        "bs": 0.04667566
      }
    ],
    "lv": 10000
  }
]
```
