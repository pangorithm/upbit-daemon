# 구독 중인 스트림 목록 조회

WebSocket 연결을 통해 구독중인 데이터 스트림 항목을 확인할 수 있습니다.

## Method

"type" 필드를 포함하는 데이터 구독 요청 메세지와 달리, 구독 중인 스트림 목록 조회 요청 메세지는 "method"필드를 포함하며 Operation 메세지의 성격을 가집니다. 데이터 구독 요청 메세지와 동일하게 요청 JSON Array에 Ticket 필드와 Format 필드와 함 Method 필드를 넣어 요청합니다.

**⚠️ Format 필드 지정 시 주의사항**

구독 중인 스트림 목록 조회 요청시 Format 필드는 실제 각 데이터를 구독 요청할 때 사용한 Format 필드와 동일한 형식으로 요청히시기 바랍니다. 다른 형식으로 요청하는 경우, 기존에 구독중이던 데이터 스트림의 요청 형식도 변경되므로 주의하십시오.

예를 들어, SIMPLE 형식으로 실시간 스트림을 수신하다가 본 요청을 DEFAULT 형식으로 요청할 경우, 구독중이던 실시간 스트림 또한 DEFAULT 형식으로 수신됩니다.

## 요청 수 제한 안내

구독 중인 스트림 목록 조회 요청도 요청 수 제한 대상에 포함됩니다.

## Request 메세지 형식

현재 구독 중인 스트림 목록을 조회하기 위해서는 사용중인 WebSocket 연결로 아래 구조의 JSON Object를 생성한 뒤 요청 메세지의 Data Type Object에 포함하여 전송해야 합니다. Ticket, Format 필드를 포함 전체 WebSocket 데이터 요청 메세지 명세는 [WebSocket 사용 안내](https://docs.upbit.com/kr/reference/websocket-guide) 문서를 참고해주세요.

| 필드명 | 타입 | 내용 | 필수 여부 | 기본 값 |
|--------|------|------|-----------|---------|
| method | String | 요청 메서드<br>`LIST_SUBSCRIPTIONS` | Required | - |

### 예시

```json
[
  {
    "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
  },
  {
    "method": "LIST_SUBSCRIPTIONS"
  }
]
```

## 응답 명세

| 필드명 | 축약형 | 내용 | 타입 | 값 |
|--------|--------|------|------|-----|
| method | mthd | 요청 메서드 | String | `LIST_SUBSCRIPTIONS` |
| result | rslt | 요청 결과 | List of Objects | - |
| result.type | rslt.ty | 데이터 타입 | String | - |
| result.codes | rslt.cds | 페어 코드 목록 | List of String | - |
| result.level | rslt.lv | 호가 모아보기 단위 | Double | - |
| ticket | tckt | 요청자를 식별할 수 있는 값 | String | - |

### 예시 - Quotation 실시간 스트림 내역

```json
{
  "method": "LIST_SUBSCRIPTIONS",
  "result": [
    {
      "type": "ticker",
      "codes": ["KRW-BTC", "KRW-ETH"]
    },
    {
      "type": "orderbook",
      "codes": ["KRW-BTC", "KRW-ETH"],
      "level": 0
    }
  ],
  "ticket": "unique uuid"
}
```

### 예시 - Exchange 실시간 스트림 내역

```json
{
  "method": "LIST_SUBSCRIPTIONS",
  "result": [
    {
      "type": "myAsset"
    },
    {
      "type": "myOrder",
      "codes": ["KRW-BTC", "KRW-ETH"]
    }
  ],
  "ticket": "unique uuid"
}
```
