# [Python] 계정주 확인 자동화

```python
from urllib.parse import unquote, urlencode
from typing import Any
from collections.abc import Mapping
import hashlib
import uuid
import jwt # PyJWT
import requests

# 본 코드를 실행하기 위해서는 JWT를 생성해 API를 호출해야 합니다.
# JWT 생성은 인증 토큰(JWT) 생성 가이드를 확인해 주시기 바랍니다.

def create_deposit_address(currency: str, net_type: str) -> Mapping:
    """
    타 거래소로 부터 입금을 받기 위한 deposit addess 생성
    """
    body = {
        "currency": currency,
        "net_type": net_type
    }
    query_string = _build_query_string(body)
    jwt_token = _create_jwt(access_key, secret_key, query_string)
    url = "https://api.upbit.com/v1/deposits/generate_coin_address"
    headers = {
        "Authorization": "Bearer {jwt_token}".format(jwt_token=jwt_token),
        "Content-Type": "application/json"
    }
    response = requests.post(url, headers=headers, json=body).json()
    return response

def get_deposit_by_uuid(uuid: str) -> Mapping: 
    """
    UUID를 사용해 특정 입금 건 조회
    """
    params = {
        "uuid": uuid
    }
    query_string = _build_query_string(params)
    jwt_token = _create_jwt(access_key, secret_key, query_string)
    url = "https://api.upbit.com/v1/deposit"
    headers = {
        "Authorization": "Bearer {jwt_token}".format(jwt_token=jwt_token),
        "Content-Type": "application/json"
    }
    response = requests.get(url, headers=headers, params=params).json()
    return response

def get_deposit_by_txid(txid: str) -> Mapping: 
    """
    txid를 사용해 특정 입금 건 조회
    """
    params = {
        "txid": txid
    }
    query_string = _build_query_string(params)
    jwt_token = _create_jwt(access_key, secret_key, query_string)
    url = "https://api.upbit.com/v1/deposit"
    headers = {
        "Authorization": "Bearer {jwt_token}".format(jwt_token=jwt_token),
        "Content-Type": "application/json"
    }
    response = requests.get(url, headers=headers, params=params).json()
    return response

def get_vasp_uuid(vasp_name: str) -> str:
    """
    사용자가 입력한 거래소 이름을 사용해 특정 거래소의 UUID 조회
    """
    params = {
        "vasp_name": vasp_name
    }
    query_string = _build_query_string(params)
    jwt_token = _create_jwt(access_key, secret_key, query_string)
    url = "https://api.upbit.com/v1/travel_rule/vasps"
    headers = {
        "Authorization": "Bearer {jwt_token}".format(jwt_token=jwt_token),
        "Content-Type": "application/json"
    }
    response = requests.get(url, headers=headers, params=params).json()
    vasp_uuid = next((item.get('vasp_uuid') for item in response if item.get('vasp_name') == vasp_name), None)
    if vasp_uuid is None:
        raise ValueError("{vasp_name} is NOT_FOUND".format(vasp_name=vasp_name))
    return vasp_uuid

def verify_travel_rule_by_uuid(deposit_uuid: str, vasp_uuid: str) -> str:
    """
    VASP와 계정주 확인 진행 (uuid 기반)
    """ 
    params = {
        "deposit_uuid": deposit_uuid,
        "vasp_uuid": vasp_uuid
    }
    query_string = _build_query_string(params)
    jwt_token = _create_jwt(access_key, secret_key, query_string)
    url = "https://api.upbit.com/v1/travel_rule/deposit/uuid"
    headers = {
        "Authorization": "Bearer {jwt_token}".format(jwt_token=jwt_token),
        "Content-Type": "application/json"
    }
    response = requests.post(url, headers=headers, json=params).json()
    verification_result = response.get('verification_result')
    if verification_result is None:
        raise ValueError("Please check the response. {response}".format(response=response))
    else:
        return verification_result

def verify_travel_rule_by_txid(deposit_txid: str, vasp_uuid: str, currency: str, net_type: str) -> str:
    """
    VASP와 계정주 확인 진행 (txid 기반)
    """
    params = {
        "txid": deposit_txid,
        "vasp_uuid": vasp_uuid,
        "currency": currency,
        "net_type": net_type
    }
    query_string = _build_query_string(params)
    jwt_token = _create_jwt(access_key, secret_key, query_string)
    url = "https://api.upbit.com/v1/travel_rule/deposit/txid"
    headers = {
        "Authorization": "Bearer {jwt_token}".format(jwt_token=jwt_token),
        "Content-Type": "application/json"
    }
    response = requests.post(url, headers=headers, json=params).json()
    verification_result = response.get('verification_result')
    if verification_result is None:
        raise ValueError("Please check the response. {response}".format(response=response))
    else:
        return verification_result
```

## 유틸 라이브러리 Import

기능 구현을 위해 필요한 모듈을 import 합니다. 별도의 설치가 필요한 모듈의 경우 `pip install <module name>` 명령어를 실행해 설치할 수 있습니다.

## 인증 토큰 생성

본 코드를 실행하기 위해서는 JWT를 생성해 API를 호출해야 합니다. JWT 생성은 [인증 토큰(JWT) 생성 가이드](python-인증-토큰jwt-생성)를 확인해 주시기 바랍니다.

## 입금 주소 생성

타 거래소에서 업비트로 디지털 자산을 입금하기 위해 필요한 입금 주소를 생성합니다. 최초 요청 시 "생성 중" 안내를, 이후 요청부터 실제 입금 주소를 조회할 수 있습니다.

## 입금 내역 확인

타 거래소에서 업비트로 입금할 때 반환받은 `UUID` 혹은 `TxID`를 사용해 업비트에서의 입금 상태를 조회합니다. 이 가이드에서는 입금 후 검증 진행 여부를 파악할 때와 검증 완료 후 입금 반영을 확인할 때 사용합니다.

## 트래블룰 지원 거래소 확인

트래블룰을 지원하는 거래소 목록을 조회하고 목록 중 사용자가 입력한 이름으로 필터링해 입금한 거래소를 특정합니다. 거래소 이름은 한국어로 입력해야 합니다.

## 트래블룰 검증

디지털 자산을 입금한 거래소와 계정주 확인을 진행하는 함수입니다. 입금 UUID 혹은 TxID, 그리고 거래소 UUID를 사용해 상대 거래소와 검증을 진행한 후, 검증 결과를 반환합니다.

## 계정주 확인 자동화

1. 입금 주소를 생성합니다.
2. 입금 트랜잭션의 `UUID` 혹은 `TxID`로 입금의 상태를 확인합니다.
3. 트래블룰 검증이 필요한 경우 거래소의 `UUID`를 구하는 함수를 실행합니다.
4. 입금 `UUID` 혹은 `TxID`, 거래소 `UUID`로 트래블룰 검증을 실행합니다.
5. 검증에 성공한 경우, 입금 건을 조회해 입금 상태를 확인합니다.
