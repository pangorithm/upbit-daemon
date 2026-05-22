# REST API 연동 Best Practice

업비트 REST API 연동 구현을 위한 가이드라인 문서로서 인증, 요청 수 제한, 에러 처리 등 실제 연동 시 참고해야 하는 구현 요구사항을 안내합니다.

## 업비트 API 연동 시 고려해야 할 사항

### 인증

인증이 필요한 Exchange API 호출 시, 요청의 파라미터 또는 본문(Body)을 기반으로 업비트 API [인증](https://docs.upbit.com/kr/reference/auth) 문서의 안내에 따라 유효한 인증 토큰을 생성하여 요청 헤더에 포함해야 합니다.

### 정상 응답 구분 및 에러 처리

REST API 응답 수신 시 HTTP 상태 코드를 기반으로 정상 응답(2xx)과 에러 응답(4xx, 5xx)을 구분한 뒤, 적절한 응답 객체 Parsing을 수행해야 합니다.

### 요청 수 제한(Rate Limit) 정책 준수

업비트 REST API는 초당 최대 [요청 수 제한(Rate Limits)](https://docs.upbit.com/kr/reference/rate-limits) 정책을 적용하고 있습니다.

### 보안 및 운영을 고려한 Logging

API 기반 매매 시스템 구현 시 호출 이력과 결과를 추적할 수 있는 구조화된 로그를 남기는 것을 권장합니다. 인증 키나 사용자 개인정보 등 민감한 데이터는 로그에 기록하지 않도록 주의해야 합니다.

## Best Practice - Python 예제

### 기본 요청 예제

```python
from collections.abc import Mapping
from urllib.parse import unquote, urlencode
import requests

class UpbitClient(object):
    def __init__(self, base_url):
        self.base_url = base_url.rstrip("/")

    def _build_url(self, path, query_string=""):
        url = "{0}/{1}".format(self.base_url, path.lstrip("/"))
        if query_string:
            url += "?{0}".format(query_string)
        return url

    def _build_query_string(self, params):
        data = params if isinstance(params, Mapping) else params
        return unquote(urlencode(data, doseq=True))

    def request_get(self, path, params=None):
        query_str = self._build_query_string(params) if params is not None else ""
        url = self._build_url(path, query_str)
        resp = requests.get(url)
        try:
            return resp.json()
        except ValueError:
            return resp.text

if __name__ == "__main__":
    client = UpbitClient("https://api.upbit.com")
    data = client.request_get("/v1/market/all")
    print(data)
```

### 인증 구현

```python
from collections.abc import Mapping
from urllib.parse import unquote, urlencode
import hashlib
import uuid
import jwt  # PyJWT
import requests

class UpbitClient(object):
    def __init__(self, base_url, access_key, secret_key):
        self.base_url = base_url.rstrip("/")
        self.access_key = access_key
        self.secret_key = secret_key
        self.public_prefixes = ("/v1/market", "/v1/ticker", "/v1/trades", "/v1/candles", "/v1/orderbook")

    def _build_url(self, path, query_string=""):
        url = "{0}/{1}".format(self.base_url, path.lstrip("/"))
        if query_string:
            url += "?{0}".format(query_string)
        return url

    def _build_query_string(self, params):
        data = params if isinstance(params, Mapping) else params
        return unquote(urlencode(data, doseq=True))

    def _create_jwt_token(self, query_string=None):
        payload = {"access_key": self.access_key, "nonce": str(uuid.uuid4())}
        if query_string:
            query_hash = hashlib.sha512(query_string.encode("utf-8")).hexdigest()
            payload["query_hash"] = query_hash
            payload["query_hash_alg"] = "SHA512"
        token = jwt.encode(payload, self.secret_key, algorithm="HS512")
        return token if isinstance(token, str) else token.decode("utf-8")

    def _requires_auth(self, path):
        return not any(path.startswith(pub) for pub in self.public_prefixes)

    def request_get(self, path, params=None):
        query_str = self._build_query_string(params) if params is not None else ""
        url = self._build_url(path, query_str)

        headers = {}
        if self._requires_auth(path):
            if not self.access_key or not self.secret_key:
                raise ValueError("인증이 필요한 API입니다. access_key와 secret_key를 설정하세요.")
            headers["Authorization"] = "Bearer {0}".format(self._create_jwt_token(query_str))

        resp = requests.get(url, headers=headers)
        try:
            return resp.json()
        except ValueError:
            return resp.text
```

### Rate Limiter 구현

```python
class RateLimiter(object):
    def __init__(self):
        self.cfg = {
            "market": (10, 1),
            "ticker": (10, 1),
            "trades": (10, 1),
            "candles": (10, 1),
            "orderbook": (10, 1),
            "default": (30, 1),
            "order": (8, 1),
            "order-cancel-all": (1, 2),
        }
        self.state = {}

    def _win_start(self, now_sec, win):
        return now_sec - (now_sec % win)

    def acquire(self, group):
        cap, win = self.cfg.get(group, (10, 1))
        now = time.time()
        now_sec = int(now)
        win_start = self._win_start(now_sec, win)
        remaining, cur_win_start = self.state.get(group, (cap, win_start))

        if cur_win_start != win_start:
            remaining, cur_win_start = cap, win_start

        if remaining <= 0:
            sleep_for = (cur_win_start + win) - now + 0.01
            if sleep_for > 0:
                time.sleep(sleep_for)
            now = time.time()
            now_sec = int(now)
            cur_win_start = self._win_start(now_sec, win)
            remaining = cap

        self.state[group] = (remaining - 1, cur_win_start)

    def update_from_header(self, header_value):
        if not header_value:
            return
        g, sec = "default", None
        try:
            for p in [s.strip() for s in header_value.split(";")]:
                if p.startswith("group="): g = p.split("=", 1)[1].strip()
                elif p.startswith("sec="): sec = int(p.split("=", 1)[1].strip())
        except Exception:
            return
        if g in self.cfg and sec is not None:
            cap, win = self.cfg[g]
            now_sec = int(time.time())
            win_start = self._win_start(now_sec, win)
            self.state[g] = (min(cap, sec), win_start)

    def mark_exhausted(self, group):
        cap, win = self.cfg.get(group, (10, 1))
        now_sec = int(time.time())
        win_start = self._win_start(now_sec, win)
        self.state[group] = (0, win_start)
```

## 마치며

* [24시간 누적 거래대금 확인](https://docs.upbit.com/kr/docs/24-hour-accumulated-trade-volume)
* [REST API 사용 및 에러 안내](https://docs.upbit.com/kr/reference/rest-api-guide)
