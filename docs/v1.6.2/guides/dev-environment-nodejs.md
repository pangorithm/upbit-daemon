# Node.js 개발 환경 설정

Node.js 환경에서 Upbit Open API를 연동하기 위한 개발 환경 설정 방법을 안내합니다.

## macOS 환경 설정

### 1. Homebrew 설치

```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew -v
```

### 2. NVM 설치

```shell
brew install nvm
```

zsh 사용자:
```shell
echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.zshrc
echo '[ -s "$(brew --prefix nvm)/nvm.sh" ] && source "$(brew --prefix nvm)/nvm.sh"' >> ~/.zshrc
source ~/.zshrc
```

bash 사용자:
```shell
echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.bash_profile
echo '[ -s "$(brew --prefix nvm)/nvm.sh" ] && source "$(brew --prefix nvm)/nvm.sh"' >> ~/.bash_profile
source ~/.bash_profile
```

### 3. Node.js 설치

```shell
nvm install --lts
node -v
```

## Windows 환경 설정

1. **Node.js 공식 웹사이트에서 설치 파일 다운로드**

* [Node.js 다운로드 바로가기](https://nodejs.org/ko/download)

설치 과정에서 **Add to PATH** 옵션이 기본 설정되어 있습니다.

## HTTP 클라이언트 라이브러리 안내

### REST API - Axios 라이브러리

```shell
npm install axios
```

```javascript
const axios = require('axios');

axios.get('https://api.upbit.com/v1/ticker', {
  params: { markets: 'KRW-BTC' },
  headers: { 'accept': 'application/json' }
})
.then(response => {
  console.log(response.data[0].trade_price);
})
.catch(error => {
  console.error(error);
});
```

### WebSocket - ws 라이브러리

```shell
npm install ws
```

```javascript
const WebSocket = require('ws');

const ws = new WebSocket('wss://api.upbit.com/websocket/v1', {
  headers: { 'accept': 'application/json' }
});

ws.on('open', () => {
  const subscribeMessage = [
    { ticket: 'test' },
    { type: 'ticker', codes: ['KRW-BTC'] }
  ];
  ws.send(JSON.stringify(subscribeMessage));
});

ws.on('message', (data) => {
  console.log('Received:', data.toString());
});

ws.on('close', () => {
  console.log('WebSocket connection closed');
});

ws.on('error', (error) => {
  console.error('WebSocket error:', error);
});
```
