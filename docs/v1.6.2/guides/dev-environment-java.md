# Java 개발 환경 설정

Java 환경에서 Upbit Open API를 연동하기 위한 개발 환경 설정 방법을 안내합니다.

## IDE(IntelliJ IDEA) 환경 설정

### 1. IntelliJ IDEA 설치

JetBrains 공식 웹사이트에서 IntelliJ IDEA를 설치합니다. Community Edition(무료), Ultimate(유료) 등 2가지 버전이 있습니다.

* [IntelliJ IDEA 다운로드 바로가기](http://jetbrains.com/idea/download)

### 2. JDK 설치

Java Development Kit(JDK)를 설치합니다. Amazon, IBM, Microsoft 등 다양한 기관에서 OpenJDK를 제공합니다.

### 3. IntelliJ IDEA 설정

1. IntelliJ IDEA를 실행하고 기존 프로젝트를 열거나 새로운 프로젝트를 생성합니다.
2. 상단 메뉴에서 File > Project Structure를 클릭합니다.
3. Platform Settings > SDKs를 선택하고, 설치한 JDK를 추가합니다.
4. Project SDK 드롭다운 메뉴에서 추가한 JDK를 선택합니다.
5. Language level을 프로젝트에 맞는 Java 버전으로 선택합니다.
6. [OK] 또는 [Apply] 버튼을 클릭해 설정을 저장합니다.

### 4. Java 환경 설정 테스트

```java
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, Java!");
    }
}
```

## HTTP 클라이언트 라이브러리 안내

### 1. OkHttp

널리 사용되는 Java/Android HTTP 클라이언트로, REST API와 WebSocket 통신 모두 지원합니다.

**Gradle**:
```Text
implementation 'com.squareup.okhttp3:okhttp:{version}'
```

**Maven**:
```xml
<dependency>
  <groupId>com.squareup.okhttp3</groupId>
  <artifactId>okhttp</artifactId>
  <version>{version}</version>
</dependency>
```

**REST API**:
```java
OkHttpClient client = new OkHttpClient();
Request request = new Request.Builder()
    .url("https://api.upbit.com/v1/ticker?markets=KRW-BTC")
    .addHeader("accept", "application/json")
    .build();

try (Response response = client.newCall(request).execute()) {
    System.out.println(response.body().string());
}
```

**WebSocket**:
```java
import okhttp3.*;
OkHttpClient client = new OkHttpClient();
Request request = new Request.Builder()
    .url("wss://api.upbit.com/websocket/v1")
    .build();

WebSocketListener listener = new WebSocketListener() {
    @Override
    public void onOpen(WebSocket webSocket, Response response) {
        String subscribeMessage = "[{\"ticket\":\"test\"},{\"type\":\"ticker\",\"codes\":[\"KRW-BTC\"]}]";
        webSocket.send(subscribeMessage);
    }
    @Override
    public void onMessage(WebSocket webSocket, String text) {
        System.out.println("Received: " + text);
    }
};
client.newWebSocket(request, listener);
```

### 2. Spring WebClient

Spring 5부터 제공되는 Reactive HTTP/WebSocket 클라이언트입니다.

**Gradle**:
```Text
implementation 'org.springframework.boot:spring-boot-starter-webflux'
```

**Maven**:
```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-webflux</artifactId>
</dependency>
```

### 3. Retrofit

OkHttp 기반의 REST API 클라이언트입니다.

**Gradle**:
```Text
implementation 'com.squareup.retrofit2:retrofit:2.11.0'
implementation 'com.squareup.retrofit2:converter-gson:2.11.0'
```

**Maven**:
```xml
<dependency>
  <groupId>com.squareup.retrofit2</groupId>
  <artifactId>retrofit</artifactId>
  <version>2.11.0</version>
</dependency>
<dependency>
  <groupId>com.squareup.retrofit2</groupId>
  <artifactId>converter-gson</artifactId>
  <version>2.11.0</version>
</dependency>
```

### 4. Java 표준 HttpClient (Java 11+)

Java 11 이상에서 내장되는 표준 HTTP 클라이언트입니다.

```java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

HttpClient client = HttpClient.newHttpClient();
HttpRequest request = HttpRequest.newBuilder()
    .uri(URI.create("https://api.upbit.com/v1/ticker?markets=KRW-BTC"))
    .header("accept", "application/json")
    .build();
HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());
System.out.println(response.body());
```
