# WebSocket 연동 Best Practice

업비트 WebSocket 연동 구현을 위한 가이드라인 문서로서 인증, 연결 관리, 요청 수 제한 등 실제 구현 시 참고해야 하는 구현 요구사항을 안내합니다.

## 업비트 WebSocket 연동시 고려해야 할 사항

### 최초 연결과 구독 요청

WebSocket 연동을 통한 데이터 조회 과정은 크게 **(1)최초 연결 생성 단계**와 **(2)구독 요청 메세지 전송** 단계의 두 단계로 나누어 진행됩니다.

1. **요청 티켓(ticket)**: 요청을 구분하기 위한 Ticket ID.
2. **구독하고자 하는 데이터 유형(type)**: candle, ticker, orderbook, trade, myOrder, myAsset
3. **데이터 형식(format)**: 기본형(DEFAULT) 또는 축약형(SIMPLE)

하나의 타입에 대해 여러 페어의 정보를 수신할 수 있으며, 동시에 여러 유형의 데이터를 구독할 수 있습니다.

### 스냅샷 데이터와 실시간 스트림 데이터

* **스냅샷**: 요청 시점의 정보를 1회 수신하는 방식.
* **실시간 스트림**: WebSocket 연결이 유지되는 동안 지속적으로 정보가 수신되는 방식.

### 연결 유지 및 재연결

업비트 WebSocket 서버는 120초 동안 데이터 송수신이 없으면 Idle Timeout으로 연결을 종료합니다. ping/pong 옵션, timeout 설정 또는 명시적 Ping 메시지 전송을 활용하여 연결을 유지할 수 있습니다.

### 인증

내 자산, 주문 및 체결 정보 구독은 `wss://api.upbit.com/websocket/v1/private` Endpoint 연동을 통해서만 가능합니다. /private 채널 연결 요청시 API Key로 생성한 [인증](https://docs.upbit.com/kr/reference/auth) 토큰이 요청 헤더에 반드시 포함되어야 합니다.

### 요청 수 제한(Rate Limit) 정책 준수

WebSocket도 연결 요청 및 메세지 전송에 대해 요청 수 제한 정책을 적용하고 있습니다.

## Best Practice - Python 예제

### 기본 WebSocket 연결 예제

```python
import threading
import websocket
import json
import time

class ThreadedWebSocketApp(threading.Thread):
    def __init__(self, url):
        threading.Thread.__init__(self)
        self.daemon = True
        self.url = url
        self.ws_app = None
        self._stop_evt = threading.Event()

    @staticmethod
    def connect(url):
        t = ThreadedWebSocketApp(url)
        t.start()
        return t

    def run(self):
        self.ws_app = websocket.WebSocketApp(
            self.url,
            on_open=self._on_open,
            on_message=self._on_message,
            on_error=self._on_error,
            on_close=self._on_close
        )
        self.ws_app.run_forever()
        self.ws_app = None

    def close(self):
        self._stop_evt.set()
        try:
            if self.ws_app:
                self.ws_app.close()
        except Exception:
            pass

    def send_message(self, message):
        try:
            if self.ws_app and self.ws_app.sock and self.ws_app.sock.connected:
                self.ws_app.send(message)
        except Exception as e:
            self.on_error(e)

    def _on_open(self, ws):
        print("Opened")

    def _on_message(self, ws, data):
        try:
            obj = json.loads(data)
            print("Received(JSON):", obj)
        except Exception:
            print("Received(raw):", data)

    def _on_error(self, ws, err):
        print("Error:", err)

    def _on_close(self, ws, code, reason):
        print("Closed")

if __name__ == "__main__":
    ws = ThreadedWebSocketApp.connect(url="wss://api.upbit.com/websocket/v1")
    try:
        time.sleep(150)
    finally:
        ws.close()
        ws.join(timeout=3)
```

### 연결 관리 - 연결 유지 및 재연결

```python
class ThreadedWebSocketApp(threading.Thread):
    def __init__(self, url, ping_interval=30, ping_timeout=10,
                max_retries=3, retry_sleep=2.0):
        threading.Thread.__init__(self)
        self.daemon = True
        self.url = url
        self.ping_interval = ping_interval
        self.ping_timeout = ping_timeout
        self.max_retries = max_retries
        self.retry_sleep = retry_sleep
        self.ws_app = None
        self._stop_evt = threading.Event()

    def run(self):
        attempts = 0
        while not self._stop_evt.is_set():
            self.ws_app = websocket.WebSocketApp(
                self.url,
                on_open=self._on_open,
                on_message=self._on_message,
                on_error=self._on_error,
                on_close=self._on_close
            )
            self.ws_app.run_forever(
                ping_interval=self.ping_interval,
                ping_timeout=self.ping_timeout,
                reconnect=int(self.retry_sleep) if self.retry_sleep else None
            )
            self.ws_app = None
```

### 인증

```python
def _create_jwt_token(self):
    if not self.access_key or not self.secret_key:
        return None
    payload = {
        "access_key": self.access_key,
        "nonce": str(uuid.uuid4())
    }
    token = jwt.encode(payload, self.secret_key, algorithm="HS512")
    return token if isinstance(token, str) else token.decode("utf-8")

def _build_headers(self):
    headers = []
    token = self._create_jwt_token()
    if token:
        headers.append("Authorization: Bearer {0}".format(token))
    return headers
```

### 요청 수 제한(Rate Limit) 관련 처리

```python
class _FixedWindowLimiter(object):
    def __init__(self, per_sec=5, per_min=100):
        self.per_sec = per_sec
        self.per_min = per_min
        self._sec_ts = 0
        self._min_ts = 0
        self._sec_used = 0
        self._min_used = 0

    def acquire(self):
        now = time.time()
        sec = int(now)
        minute = int(now // 60)

        if sec != self._sec_ts:
            self._sec_ts = sec
            self._sec_used = 0
        if minute != self._min_ts:
            self._min_ts = minute
            self._min_used = 0

        if self._sec_used >= self.per_sec:
            sleep_for = (self._sec_ts + 1) - now + 0.001
            if sleep_for > 0:
                time.sleep(sleep_for)

        if self._min_used >= self.per_min:
            sleep_for = ((self._min_ts + 1) * 60) - now + 0.001
            if sleep_for > 0:
                time.sleep(sleep_for)

        self._sec_used += 1
        self._min_used += 1
```

## 마치며

* [REST API 연동 Best Practice](https://docs.upbit.com/kr/docs/rest-api-best-practice)
