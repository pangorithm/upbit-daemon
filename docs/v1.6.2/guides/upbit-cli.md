# Upbit CLI

터미널 환경에서 업비트 API를 호출하는 공식 CLI 도구의 개념과 사용 방법을 안내합니다.

## Upbit CLI란 무엇인가요?

CLI(Command Line Interface)는 텍스트 기반의 명령줄 인터페이스로, 터미널에서 명령어를 입력하여 기능을 실행하는 방식입니다.
Upbit CLI는 터미널 환경에서 Upbit API를 호출할 수 있도록 제공되는 공식 명령줄 도구입니다.

Upbit CLI를 사용하면 별도의 코드 작성 없이 업비트 개발자센터에서 지원하는 시세 조회, 계좌 조회, 주문 조회 등의 작업을 수행할 수 있습니다.

## 언제 사용하나요?

* 터미널에서 업비트 API를 빠르게 호출하고 결과를 확인하고 싶은 경우
* 반복적인 API 호출을 스크립트로 구성하려는 경우
* API 응답을 검증하거나 문제를 디버깅하려는 경우

## 사전 준비

* Node.js 및 npm 또는 Go 환경
* 업비트 Open API Key (Access Key와 Secret Key)

## 설치

### npm으로 설치

```bash
npm install -g @upbit-official/upbit-cli
```

### Go로 설치

```bash
go install github.com/upbit-official/upbit-cli/cmd/upbit@latest
```

### 설치 확인

```bash
upbit --version
```

## 인증 정보 설정

### macOS / Linux

```bash
export UPBIT_ACCESS_KEY=<your-access-key>
export UPBIT_SECRET_KEY=<your-secret-key>
```

### Windows PowerShell

```powershell
$env:UPBIT_ACCESS_KEY="your-access-key"
$env:UPBIT_SECRET_KEY="your-secret-key"
```

명령 실행 시 플래그로 API Key를 직접 전달할 수도 있습니다.

```bash
upbit accounts list \
  --access-key "$UPBIT_ACCESS_KEY" \
  --secret-key "$UPBIT_SECRET_KEY"
```

## 정상 동작 확인

```bash
# 공개 API
upbit trading-pairs list --is-details=false

# 인증 API
upbit accounts list
```

## 주요 사용 예시

### 시세 조회

```bash
upbit tickers list-by-trading-pairs --markets "KRW-BTC"
```

### 마켓 정보 조회

```bash
upbit trading-pairs list
```

### 주문 및 계좌 조회

```bash
upbit orders list-open --market KRW-BTC
upbit accounts list
```

### 입출금 조회

```bash
upbit deposits list
upbit withdrawals list
```

## 출력 형식과 데이터 처리

### 출력 형식 설정

```bash
upbit accounts list --format json
upbit trading-pairs list --format yaml
```

지원 형식: `auto`, `explore`, `json`, `jsonl`, `pretty`, `raw`, `yaml`

### 데이터 필터링

`--transform` 옵션과 GJSON 구문을 사용하여 필요한 데이터만 추출할 수 있습니다.

```bash
# 마켓 코드만 추출
upbit trading-pairs list --transform "#.market"

# BTC 현재가만 추출(GJSON)
upbit tickers list-by-trading-pairs \
  --markets "KRW-BTC" \
  --transform "0.trade_price"
```

### 페이지네이션

```bash
upbit orders list-open --max-items 20
upbit deposits list --max-items 20
```

### 디버그 모드

```bash
upbit accounts list --debug
```

## 참고

* [Upbit CLI ReadMe](https://github.com/upbit-official/upbit-cli)
* [Upbit CLI 예제 코드](https://github.com/upbit-official/upbit-cli/tree/main/examples)
