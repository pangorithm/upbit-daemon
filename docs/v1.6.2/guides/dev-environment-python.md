# Python 개발 환경 설정

Python 환경에서 Upbit Open API를 연동하기 위한 개발 환경 설정 방법을 안내합니다.

## macOS 환경 설정

### 1. Homebrew 설치

```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew -v
```

### 2. Python 설치

```shell
brew install python
python3 --version
```

### 3. 가상 환경 설정

```shell
python3 -m venv .venv
source .venv/bin/activate
deactivate
```

## Windows 환경 설정

### 1. Python 공식 웹사이트에서 설치 파일 다운로드

* [Python 다운로드 바로가기](https://www.python.org/downloads/)

설치 과정에서 **Add Python to PATH** 옵션을 선택하면 별도의 환경 변수 설정 없이 Python을 바로 사용할 수 있습니다.

### 2. 가상 환경 설정

```shell
python -m venv .venv
.venv\Scripts\activate
deactivate
```

## HTTP 클라이언트 라이브러리 안내

### REST API - requests 라이브러리

1. **설치**

```shell
pip install requests
```

2. **기본 사용법**

```python
import requests

url = "https://api.upbit.com/v1/ticker?markets=KRW-BTC"
response = requests.get(url)
data = response.json()
print(data[0]["trade_price"])
```

### WebSockets - websocket-client, websockets 라이브러리

1. **설치**

```shell
pip install websocket-client
pip install websockets
```

2. **기본 사용법 (websocket-client)**

```python
import websocket
import json

def on_message(ws, message):
    print("Received:", message)

ws = websocket.WebSocketApp(
    "wss://api.upbit.com/websocket/v1",
    on_message=on_message
)

subscribe_message = [
    {"ticket":"test"},
    {"type":"ticker","codes":["KRW-BTC"]}
]

def on_open(ws):
    ws.send(json.dumps(subscribe_message))

ws.on_open = on_open
ws.run_forever(ping_interval=30, ping_timeout=10, reconnect=2)
```
