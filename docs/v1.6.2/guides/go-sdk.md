# Go SDK

Go 환경에서 Upbit SDK를 사용하여 Upbit API를 호출하기 위한 개발 환경 설정 방법을 안내합니다.

## SDK 공식 문서

Upbit Go SDK 관련 문서는 아래에서 확인할 수 있습니다.

* [시작 가이드](https://github.com/upbit-official/upbit-sdk-go/blob/main/README_KR.md) — Upbit Go SDK의 설치 및 기본 사용 방법
* [Upbit SDK API Reference](https://github.com/upbit-official/upbit-sdk-go/blob/main/api.md) — SDK가 지원하는 전체 API 목록
* [SDK 예제 코드](https://github.com/upbit-official/upbit-sdk-go/tree/main/examples) — 예제 코드 및 상세 사용법

## 사전 준비

* Go 1.25 이상
* Upbit Open API Key (Access Key와 Secret Key)

## 프로젝트 생성

```bash
mkdir <project-name>
cd <project-name>
go mod init <module-path>
```

예시:

```bash
mkdir upbit-go-example
cd upbit-go-example
go mod init github.com/username/upbit-go-example
```

## SDK 설치

```bash
go get github.com/upbit-official/upbit-sdk-go

# 특정 버전 고정
go get github.com/upbit-official/upbit-sdk-go@v0.9.0
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

## 기본 사용 예제

```go
package main

import (
	"context"
	"fmt"
	"os"

	"github.com/upbit-official/upbit-sdk-go"
	"github.com/upbit-official/upbit-sdk-go/option"
)

func main() {
	client := upbit.NewClient(
		option.WithAccessKey(os.Getenv("UPBIT_ACCESS_KEY")),
		option.WithSecretKey(os.Getenv("UPBIT_SECRET_KEY")),
	)

	accounts, err := client.Accounts.List(context.TODO())
	if err != nil {
		panic(err.Error())
	}

	fmt.Printf("%+v\n", accounts)
}
```

## 예제 코드 실행

### 인증이 필요 없는 예제

| 예제 | 설명 |
| --------------- | ------------------------ |
| `quotation.go` | 시세, 캔들, 체결, 호가 조회 |
| `indicators.go` | Quotation 데이터를 활용한 지표 계산 |

### 인증이 필요한 예제

| 예제 | 설명 |
| -------------------- | --------------------------- |
| `orders.go` | 주문 생성, 조회, 취소 흐름 |
| `orders_validate.go` | 주문 생성 테스트 API를 활용한 주문 유형 검증 |
| `deposits.go` | 입금 주소 및 입금 내역 관리 |
| `withdrawals.go` | 출금 가능 정보 및 출금 흐름 |
| `dca.go` | 정기적 시장가 매수 예제 |
| `tp_sl.go` | 익절/손절 자동 매도 예제 |

## Dry run 모드

일부 예제는 기본적으로 Dry run 모드로 동작합니다.

```bash
DRY_RUN=false UPBIT_ACCESS_KEY=<key> UPBIT_SECRET_KEY=<secret> go run examples/orders.go
```

## 고급 기능

### 페이지네이션

```go
iter := client.Orders.ListOpenAutoPaging(context.TODO(), upbit.OrderListOpenParams{})

for iter.Next() {
	order := iter.Current()
	fmt.Printf("%+v\n", order)
}

if err := iter.Err(); err != nil {
	panic(err.Error())
}
```

### 요청 옵션

```go
client.Accounts.List(
	context.TODO(),
	option.WithRequestTimeout(20*time.Second),
)
```

### 에러 처리

```go
accounts, err := client.Accounts.List(context.TODO())
if err != nil {
	var apierr *upbit.Error
	if errors.As(err, &apierr) {
		fmt.Println(apierr.StatusCode)
	}
	panic(err.Error())
}
```
