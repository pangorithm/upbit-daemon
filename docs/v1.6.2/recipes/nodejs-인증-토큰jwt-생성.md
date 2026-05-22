# [Node.js] 인증 토큰(JWT) 생성

```javascript
const axios = require('axios');
const crypto = require('crypto');
const jwt = require('jsonwebtoken');
const { v4: uuidv4 } = require('uuid');

// 필수 정보 입력
const ACCESS_KEY = "<YOUR_ACCESS_KEY>";
const SECRET_KEY = "<YOUR_SECRET_KEY>"; // 안전하게 관리하세요
const BASE_URL = 'https://api.upbit.com';

/**
 * SHA512 해시 함수
 */
function sha512(text) {
  return crypto.createHash('sha512').update(text, 'utf8').digest('hex');
}

/**
 * Dictionary 파라미터를 쿼리 문자열 형식으로 변환
 */
function buildQueryStrings(params) {
  const encoded = new URLSearchParams(
    Object.entries(params).flatMap(([key, value]) =>
      Array.isArray(value) ? value.map((v) => [key, v]) : [[key, value]]
      )
    ).toString();
  const raw = decodeURIComponent(encoded);
  return { encoded, raw };
}

/**
 * JWT 토큰 생성
 */
function createJwtToken(accessKey, secretKey, queryString = '') {
  const payload = {
    access_key: accessKey,
    nonce: uuidv4(),
  };
  if (queryString) {
    payload.query_hash = sha512(queryString);
    payload.query_hash_alg = 'SHA512';
  }
  return jwt.sign(payload, secretKey, { algorithm: 'HS512' });
}

/**
 * GET 파라미터 없는 요청 예제
 */
async function getAccount() {
  const token = createJwtToken(ACCESS_KEY, SECRET_KEY);
  const headers = {
      Authorization: `Bearer ${token}`,
      Accept: 'application/json',
  };

  try {
      const res = await axios.get(`${BASE_URL}/v1/accounts`, { headers });
      console.log('[GET] Status:', res.status);
      console.log(res.data);
  } catch (err) {
      console.error('[GET] Error:', err.response?.data || err.message);
  }
}

/**
 * GET 파라미터 있는 요청 예제
 */
async function getOpenOrders() {
  const query = {
    states: ['wait', 'watch'],
    page: 1,
    limit: 10,
  };

  const { encoded, raw } = buildQueryStrings(query);
  const token = createJwtToken(ACCESS_KEY, SECRET_KEY, raw);
  const headers = {
    Authorization: `Bearer ${token}`,
    Accept: 'application/json',
  };

  try {
    const res = await axios.get(`${BASE_URL}/v1/orders/open?${encoded}`, {  headers });
    console.log('[GET] Status:', res.status);
    console.log(res.data);
  } catch (err) {
    console.error('[GET] Error:', err.response?.data || err.message);
  }
}

/**
 * POST 요청 예제
 */
async function placeOrder() {
  const body = {
    market: 'KRW-BTC',
    side: 'bid',
    volume: '0.001',
    price: '5000000',
    ord_type: 'limit',
  };  

  const { raw } = buildQueryStrings(body);
  const token = createJwtToken(ACCESS_KEY, SECRET_KEY, raw);
  const headers = {
    Authorization: `Bearer ${token}`,
    Accept: 'application/json',
    'Content-Type': 'application/json',
  };

  try {
    // 아래 주석처리된 부분 실행시 실제 주문이 발생하므로 실행 전 반드시 확인하세요.
    // const res = await axios.post(`${BASE_URL}/v1/orders`, body, { headers });
    // console.log('[POST] Status:', res.status);
    // console.log(res.data);
    console.log('[POST] Request prepared but not sent (order disabled for safety).');
  } catch (err) {
    console.error('[POST] Error:', err.response?.data || err.message);
  }
}

// 메인 실행
(async () => {
  await getAccount();
  await getOpenOrders();
  await placeOrder();
})();
```

## 유틸 라이브러리 Import

인증 토큰을 생성하기 위해 필요한 모듈을 import 합니다. 별도의 설치가 필요한 모듈의 경우 `npm install <module name>` 명령어를 실행해 설치할 수 있습니다.

## 필수 정보 입력

인증 토큰 생성에 필요한 Access Key와 Secret Key, 그리고 요청을 전송할 엔드포인트를 정의합니다.

## 해시 함수 정의

사용자가 입력한 문자열을 해시하는 함수입니다. 쿼리 문자열로 인코딩 된 파라미터를 해시하기 위해 사용합니다.

## 파라미터 인코딩

입력받은 파라미터 객체를 쿼리 문자열로 변환하는 함수입니다. 배열 값은 [] 형식으로 직렬화하며, 키는 URL 인코딩하지 않고 값만 인코딩합니다. 변환 결과로, URL 요청에 바로 사용할 수 있는 인코딩된 문자열(encoded)과 JWT 생성 시 사용할 디코딩된 원본 문자열(raw)을 반환합니다.

## JWT 생성

JWT는 payload를 사용해 생성합니다. 기본 payload는 Access Key와 nonce를 가진 객체로 사용자의 파라미터 입력 여부에 따라 payload의 값이 달라집니다.

## API 호출로 JWT 동작 확인

생성한 JWT가 정상적으로 동작하는지 확인할 수 있는 예시 코드 입니다. 다음 3가지 요청을 통해 JWT의 동작을 확인할 수 있습니다.

1. 파라미터 없는 GET 요청
2. 파라미터를 입력하는 GET 요청
3. Body 파라미터를 입력하는 POST 요청

단, POST 요청은 주석을 해제하고 실행해야 합니다. 또한 POST 요청 시 실제 주문이 생성될 수 있으므로 실행하기 전 반드시 확인 후 실행하시기 바랍니다.
