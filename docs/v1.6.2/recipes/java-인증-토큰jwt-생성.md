# [Java] 인증 토큰(JWT) 생성

```java
package com.upbit.openapi.test;

import com.auth0.jwt.JWT;
import com.auth0.jwt.JWTCreator;
import com.auth0.jwt.algorithms.Algorithm;
import com.google.gson.Gson;
import java.io.IOException;
import java.math.BigInteger;
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.UUID;
import okhttp3.MediaType;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.RequestBody;
import okhttp3.Response;

public class Auth {

    private static Gson gson = new Gson();

    /**
     * 문자열을 입력 받아 Hash를 생성하는 예제
     */
    public static String sha512(String input) throws NoSuchAlgorithmException {
        // input은 non-null의 유효한 값으로 가정
        MessageDigest md = MessageDigest.getInstance("SHA-512");
        md.update(input.getBytes(StandardCharsets.UTF_8));

        return HexFormat.of().formatHex(md.digest());
    }

    /**
     * Access Key, Secret Key, QueryString을 입력받아 JWT Token을 생성하는 예제
     */
    private static String createJwt(String accessKey, String secretKey, String queryString)
        throws NoSuchAlgorithmException {
        // accessKey, secretKey, payload는 모두 non-null의 유효한 값으로 가정
        byte[] secretKeyBytes = secretKey.getBytes(StandardCharsets.UTF_8);
        Algorithm algorithm;

        try {
            algorithm = Algorithm.HMAC512(secretKeyBytes);
        } finally {
            Arrays.fill(secretKeyBytes, (byte) 0);
        }

        // Build JWT with claims
        JWTCreator.Builder builder = JWT.create()
            .withHeader(Collections.singletonMap("alg", "HS512"))
            .withClaim("access_key", accessKey)
            .withClaim("nonce", UUID.randomUUID().toString());

        // queryString이 있는 경우 페이로드에 Hash 추가
        if (queryString != null && !queryString.isEmpty()) {
            String queryHash = sha512(queryString);
            builder.withClaim("query_hash", queryHash);
            builder.withClaim("query_hash_alg", "SHA512");
        }
        return builder.sign(algorithm);
    }

    /**
     * 요청의 Json Body를 QueryString으로 변환하는 예제
     */
    public static String jsonToQueryString(String jsonString) {
        if (jsonString == null || jsonString.isEmpty()) {
            return "";
        }

        Map<String, Object> bodyMap = gson.fromJson(jsonString, Map.class);
        if (bodyMap != null && !bodyMap.isEmpty()) {
            List<String> queryElements = new ArrayList<>();
            for (Map.Entry<String, Object> entry : bodyMap.entrySet()) {
                if (entry.getValue() != null) {
                    try {
                        String encodedKey = URLEncoder.encode(entry.getKey(), "UTF-8");
                        String encodedValue = URLEncoder.encode(String.valueOf(entry.getValue()),
                            "UTF-8");

                        encodedKey = encodedKey.replace("%5B", "[").replace("%5D", "]");

                        queryElements.add(encodedKey + "=" + encodedValue);
                    } catch (Exception e) {
                        throw new RuntimeException("Encoding failed", e);
                    }
                }
            }
            return String.join("&", queryElements);
        }
        return "";
    }

    /**
     * Map<String, Object>를 URL 인코딩된 Query String 변환하는 예제
     */
    public static String buildQueryString(Map<String, Object> params) {
        if (params == null || params.isEmpty()) {
            return "";
        }

        List<String> components = new ArrayList<>();

        for (Map.Entry<String, Object> entry : params.entrySet()) {
            String key = entry.getKey();
            Object value = entry.getValue();

            if (value == null) {
                continue;
            }

            List<Object> values;
            if (value instanceof List) {
                values = (List<Object>) value;
            } else {
                values = Collections.singletonList(value);
            }

            for (Object val : values) {
                try {
                    String encodedKey = URLEncoder.encode(
                        key.endsWith("[]") ? key : key + "[]", StandardCharsets.UTF_8
                    ).replace("%5B", "[").replace("%5D", "]");

                    String encodedVal = URLEncoder.encode(String.valueOf(val),
                        StandardCharsets.UTF_8);
                    components.add(encodedKey + "=" + encodedVal);
                } catch (Exception e) {
                    throw new RuntimeException("Encoding failed", e);
                }
            }
        }

        return String.join("&", components);
    }


    public static void main(String[] args) throws IOException, NoSuchAlgorithmException {
        String baseUrl = "https://api.upbit.com";
        String accessKey = "<YOUR_ACCESS_KEY>";
        String secretKey = "<YOUR_SECRET_KEY>"; // 실제로는 안전하게 로드하거나 주입하세요.

        OkHttpClient client = new OkHttpClient();

        // 파라미터가 있는 GET 요청 예시
        Map<String, Object> queryParams = new HashMap<>();
        queryParams.put("states[]", Arrays.asList("wait", "watch"));
        queryParams.put("limit", 100);

        String queryString = buildQueryString(queryParams);
        String jwtTokenGet = createJwt(accessKey, secretKey, queryString);

        Request getRequest = new Request.Builder()
            .url(baseUrl + "/v1/orders/open?" + queryString)
            .get()
            .addHeader("Accept", "application/json")
            .addHeader("Authorization", "Bearer " + jwtTokenGet)
            .build();

        Response response = client.newCall(getRequest).execute();
        System.out.println(response.code());
        System.out.println(Objects.requireNonNull(response.body()).string());

        // POST 요청 예시
        final MediaType JSON = MediaType.parse("application/json; charset=utf-8");
        String jsonBody = "{\"market\":\"KRW-BTC\",\"side\":\"bid\",\"volume\":\"0.0001\",\"price\":\"50000000\",\"ord_type\":\"limit\"}";
        String queryStringBody = jsonToQueryString(jsonBody);
        String jwtTokenPost = createJwt(accessKey, secretKey, queryStringBody);

        Request postRequest = new Request.Builder()
            .url(baseUrl + "/v1/orders")
            .post(RequestBody.create(jsonBody, JSON))
            .addHeader("Accept", "application/json")
            .addHeader("Authorization", "Bearer " + jwtTokenPost)
            .build();

        // 아래 주석처리된 부분 실행시 실제 주문이 발생하므로 실행 전 반드시 확인하세요.
        response = client.newCall(postRequest).execute();
        System.out.println(response.code());
        System.out.println(Objects.requireNonNull(response.body()).string());
    }
}
```

## 유틸 라이브러리 Import

인증 토큰을 생성하기 위해 필요한 모듈을 import 합니다.

## Input 해시

사용자가 입력한 문자열을 해시하는 함수입니다. 파라미터를 URL 인코딩한 쿼리 문자열을 해시하기 위해 사용합니다.

## JWT 생성

JWT는 payload를 사용해 생성합니다. 기본 payload는 Access Key와 nonce를 가진 객체로 사용자의 파라미터 입력 여부에 따라 payload의 값이 달라집니다.

## 바디 파라미터 변환

바디 파라미터를 쿼리 문자열로 변환하는 함수입니다.

## Map 타입 파라미터 변환

Map 타입의 파라미터를 쿼리 문자열로 변환하는 함수입니다.

## API 호출로 JWT 동작 확인

생성한 JWT가 정상적으로 동작하는지 확인할 수 있는 예시 코드 입니다. 다음 3가지 요청을 통해 JWT의 동작을 확인할 수 있습니다.

1. 파라미터 없는 GET 요청
2. Query 파라미터를 입력하는 GET 요청
3. Body 파라미터를 입력하는 POST 요청

단, POST 요청은 주석을 해제하고 실행해야 합니다. 또한 POST 요청 시 실제 주문이 생성될 수 있으므로 실행하기 전 반드시 확인 후 실행하시기 바랍니다.
