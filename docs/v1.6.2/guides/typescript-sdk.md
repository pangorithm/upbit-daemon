# TypeScript SDK

TypeScript 환경에서 Upbit SDK 라이브러리를 사용해 업비트 API를 호출하는 방법을 안내합니다.

## SDK 공식 문서

Upbit TypeScript SDK는 공식 문서를 제공합니다. 아래 링크를 클릭해 공식 웹사이트로 이동할 수 있습니다.

* [시작 가이드](https://github.com/upbit-official/upbit-sdk-typescript/blob/main/README_KR.md) — Upbit TypeScript SDK의 설치 및 기본 사용 방법
* [Upbit SDK API Reference](https://github.com/upbit-official/upbit-sdk-typescript/blob/main/api.md) — SDK가 지원하는 전체 API 목록
* [SDK 예제 코드](https://github.com/upbit-official/upbit-sdk-typescript/tree/main/examples) — 예제 코드 및 상세 사용법

## TypeScript 연동 가이드

* 최소 버전: TypeScript 4.9 이상
* Node.js 22 LTS 이상 권장

### 프로젝트 초기화 및 SDK 설치

#### 1. 프로젝트 디렉토리 생성

```bash
mkdir upbit_sdk_project
cd upbit_sdk_project
npm init -y
```

#### 2. TypeScript 및 SDK 설치

```bash
npm install @upbit-official/upbit-sdk
npm install -D typescript ts-node @types/node
```

#### 3. TypeScript 설정 파일 생성

```bash
npx tsc --init
```

### 클라이언트 인스턴스 설정

> **환경변수를 통한 인증**: `UPBIT_ACCESS_KEY`와 `UPBIT_SECRET_KEY` 환경변수를 설정하면 클라이언트 생성 시 키를 직접 전달하지 않아도 자동으로 인증 정보를 읽어옵니다.

```typescript
import Upbit from '@upbit-official/upbit-sdk';

const client = new Upbit({
  accessKey: process.env['UPBIT_ACCESS_KEY'],
  secretKey: process.env['UPBIT_SECRET_KEY'],
});
```

### 인스턴스 설정 확인

#### 인증이 필요한 API 호출

```typescript
import Upbit from '@upbit-official/upbit-sdk';

async function getBalance(): Promise<void> {
  const client = new Upbit({
    accessKey: process.env['UPBIT_ACCESS_KEY'],
    secretKey: process.env['UPBIT_SECRET_KEY'],
  });
  const result = await client.accounts.list();
  console.log(result);
}

async function main(): Promise<void> {
  await getBalance();
}

main().catch(console.error);
```

#### 인증 없이 API 호출

```typescript
import Upbit from '@upbit-official/upbit-sdk';

async function listTradingPairsDefault(): Promise<void> {
  const client = new Upbit();
  const result = await client.tradingPairs.list({
    is_details: false,
  });
  console.log(result);
}

async function main(): Promise<void> {
  await listTradingPairsDefault();
}

main().catch(console.error);
```

### 에러 핸들링

```typescript
const accounts = await client.accounts.list().catch(async (err) => {
  if (err instanceof Upbit.APIError) {
    console.log(err.status);  // 404
    console.log(err.name);    // "not_found"
    console.log(err.message); // "404 [not_found] no Route matched"
    console.log(err.headers); // {server: 'nginx', ...}
  } else {
    throw err;
  }
});
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

### 추가 기능

#### 재시도 설정

```typescript
// 모든 요청의 기본값 설정
const client = new Upbit({
  maxRetries: 0, // 기본값은 2
});

// 요청별 설정
await client.accounts.list({
  maxRetries: 5,
});
```

#### 타임아웃 설정

```typescript
// 모든 요청의 기본값 설정
const client = new Upbit({
  timeout: 20 * 1000, // 20초 (기본값은 1분)
});

// 요청별 설정
await client.accounts.list({
  timeout: 5 * 1000,
});
```

#### 자동 페이지네이션

```typescript
async function fetchAllOrders(params) {
  const allOrders = [];
  for await (const order of client.orders.listOpen()) {
    allOrders.push(order);
  }
  return allOrders;
}

// 수동 페이지네이션
let page = await client.orders.listOpen();
for (const order of page.items) {
  console.log(order);
}

while (page.hasNextPage()) {
  page = await page.getNextPage();
}
```
