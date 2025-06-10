# Modbus RTU

이 크래이트는 Modbus RTU 표준 프로토콜 및 일부 커스텀 프로토콜의 패킷 생성/분석 자동화를 위한 기능을 제공합니다.
이 크래이트는 기본적으로 임베디드 환경을 대상으로 설계되었으므로, no-std 환경에서도 동작 가능합니다.

> 현재 지원하는 표준 기능은 아래와 같습니다.
> - `0x03`: Read Holding Registers
> - `0x04`: Read Input Registers
> - `0x06`: Write Single Register
> - `0x10`: Write Multiple Registers

> 추후 버전에서 아래 기능을 지원할 예정입니다.
> - `0x01`: Read Coils
> - `0x02`: Read Discrete Inputs
> - `0x05`: Write Single Coil
> - `0x0F`: Write Multiple Coils
> - `0x16`: Mask Write Register
> - `0x17`: Read/Write Multiple registers

# 사용 방법

### 1. Feature 활성화

우선, 개발 대상에 맞게 `Cargo.toml`에서 features를 활성화 해야합니다.

만약, 개발 대상이 하위 장치(Slave)와 통신하는 **상위 장치(Master)**라면, 아래와 같이 "master" feature를 활성화 해야합니다.
```toml
[dependencies]
modbus-rtu = { version = "x.x.x", features = ["master"] }
```

만약, 개발 대상이 상위 장치(Master)와 통신하는 **하위 장치(Slave)**라면, 아래와 같이 "slave" feature를 활성화 해야합니다.
```toml
[dependencies]
modbus-rtu = { version = "x.x.x", features = ["slave"] }
```

만약, **Gateway** 장치 또는 본 장치의 하위 장치와 상위 장치를 연결하는 어떠한 장치라면 다음과 같이 두 features를 모두 활성화 해야합니다.
```toml
[dependencies]
modbus-rtu = { version = "x.x.x", features = ["master", "slave"] }
```

---

### 2. Master 장치 구현 예시

우선 요청을 생성해야합니다.

이 크래이트는 반복적인 요청을 재사용 하기 위해 전체 요청 패킷을 다음과 같이 구분짓고있습니다.
```
+------------------------ Request ----------------------+
|                                                       |
|      +------------ Function ------------+             |
|      |                                  |             |
  0x01   0x04   0x12   0x34   0x00   0x04   0xB5   0x7F  
  ^^^^   ^^^^   ^^^^^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^
   |      |      |> Data Bytes               |> CRC Bytes
   |      |
   |      |> Function Code
   |
   |> Modbus Slave ID
```
- Function은 어떤 장치에게 보낼 기능 요청에 대한 정보만을 가지고있습니다.
- Packet은 보낼 기능 요청과 그 요청을 받을 장치에 대한 정보를 포함하고 있습니다.

---

먼저 Request Function을 생성해야합니다. `0x1234`부터 4개의 Input Registers를 읽는 요청을 경우 다음과 같이 작성할 수 있습니다.
```rust
use modbus_rtu::Function;
use modbus_rtu::function::ReadInputRegisters;

let request = Function::ReadInputRegisters(ReadInputRegisters::new(0x1234, 4));
```

만약 같은 Request Function을 반복적으로 사용한다면 다음과 같이 상수로 만들 것을 추천합니다.
```rust
const READ_SENSOR_VALUES: Function = Function::ReadInputRegisters(ReadInputRegisters::new(0x1234, 4));
```

그 다음으로 요청 패킷을 만들어야합니다.
위 요청을 받을 장치의 ID(주소)가 `1`이라고 한다면 다음과 같이 작성할 수 있습니다.
```rust
use modbus_rtu::Request;

let request = Request::new(0x01, READ_SENSOR_VALUES);
```

그리고 다음과 같이 전송할 수 있습니다.
```rust
let mut buf: [u8; 256] = [0x00; 256];

// 실제 전송 가능한 바이트 생성
let bytes: &[u8] = request.to_bytes(&mut buf);

// 전송 (예시)
write_all(&bytes);
```

요청을 보낸 후, 타임아웃에 대해서 사용자가 직접 처리 해야합니다.

수신 완료 후 수신한 데이터에 대한 슬라이스 `bytes`가 있을 때 다음과 같이 해석을 시작할 수 있습니다.
```rust
use modbus_rtu::Response;
use modbus_rtu::packet::Error;

let mut buf: [u8; 256] = [0x00; 256]; // 해석 결과를 저장할 버퍼
let analyze_result: Result<Response, Error> = Response::from_bytes(&request, &bytes, &mut buf);

let response = match analyze_result {
    Ok(response) => response,
    Err(e) => {
        // 에러에 대한 처리
        return;
    },
};

// 해석 단계에서 제공한 u8 버퍼를 참조하는 u16 슬라이스를 반환합니다.
let values: &[u16] = match response {
    Response::ReadInputRegisters(read_input_registers) => read_input_registers.values(),
    _ => unreacable!(),
}
```

기능 코드에 따라 반환되는 값이 다릅니다.
```rust
// Input 레지스터 읽기 및 Holding 레지스터 읽기 -> u16 슬라이스를 반환합니다.
let values: &[u16] = read_holding_registers.values();
let values: &[u16] = read_input_registers.values();

// 단일 레지스터 쓰기 및 여러 레지스터 쓰기 -> 반환 값이 없습니다. Response::from_bytes()의 결과가 Ok이면 성공입니다.
let is_success = Response::from_bytes(..).is_ok();
```

---

### 3. Slave 장치 구현 예시

우선 PDU(Protocol Data Unit) 구조를 정의 해야합니다.

이 크래이트는 Modbus RTU Protocol의 PDU를 Separated-Block 스타일로 정의합니다.
즉 Holding register `0x1234`와 Input Register `0x1234`는 같은 주소를 가지지만, 서로 다른 데이터를 가집니다.

다음과 같이 Holding Registers를 정의할 수 있습니다.
```rust
use modbus_rtu::slave::{DataStruct, DataModel};

// 유효한 Holding Register 주소 목록입니다.
const HOLDING_REGISTER_STRUCT: DataStruct<3> = DataStruct::new([
    0x1234,
    0x5678,
    0x9ABC,
]);

// 모든 값을 0으로 초기화합니다.
let mut holding_registers = DataModel::new(&HOLDING_REGISTER_STRUCT, [0x0000_u16; 3]);
```

Input registers도 동일하게 정의할 수 있습니다.

---

데이터를 수신하는 부분은 하드웨어 요구 사항에 따라 직접 구현 해야합니다.

수신한 데이터 슬라이스를 `bytes`라고 할 때, 다음과 같이 패킷 분석을 시작할 수 있습니다.
아래는 단순화된 예시입니다.
```rust
use modbus_rtu::{Request, Response, Exception};
use modbus_rtu::response::*;
use modbus_rtu::packet::Error;

let mut buf: [u8; 256] = [0x00; 256]; // 해석 결과를 저장할 버퍼
let analyze: Result<Request, Error> = Request::from_bytes(&bytes, &mut buf);

// 에러일 경우
if let Err(e) = analyze {
    match e {
        // 무시. 응답이 필요 없습니다.
        Error::NotMyId(_) => return,

        // 아래 두 경우는 통신이 불안정할 가능성에 해당합니다. 디버깅에 사용할 수 있습니다. 응답이 필요 없습니다.
        Error::TooShort(len) => return,
        Error::CrcMismatch { expected, received } => return,

        // 예외 코드. 응답이 필요합니다.
        Error::Exeption { fc, exception } => {
            // 고정된 길이의 예외 응답
            let bytes: [u8; 5] = exception.to_bytes(fc);

            // broadcast라면 응답할 필요 없음
            if bytes[0] == 0x00 {
                return;
            }

            // 예외 코드 응답 처리
            write_all(&bytes);
            return;
        },
    }
}

// 정상 요청일 경우
if let Ok(request) = analyze {
    let function: Function = request.get_function();
    let is_broadcasting: bool = request.get_modbus_id() == 0x00;   // broadcast일 경우 응답이 필요 없으므로

    // 요청에 따라 알맞게 처리합니다.
    let process: Result<Response, Error> = match function {
        Function::ReadHoldingRegisters { start, quantity } => {
            // u8 버퍼를 넣으면 그 버퍼를 가르키는 u16 슬라이스를 반환합니다.
            // 유효하지 않은 주소 영역에 대한 읽기 시도가 있으면 바로 에러를 반환합니다.
            let values: &[u16] = holding_registers.copy_values(start, quantity, &mut buf)?;

            let response = Response::ReadHoldingRegisters(ReadHoldingRegisters::new(modbus_id, &values));
            Ok(response)
        },

        Function::ReadInputRegisters { start, quantity } => {
            // 위와 동일합니다.
        },

        Function::WriteSingleRegister { register, data } => {
            // 유효한 주소인지 확인합니다.
            // 유효하지 않은 주소일 경우 바로 에러를 반환합니다.
            holding_registers.check_address(register)?;

            // 여기서 커스텀 동작을 할 수 있습니다.
            // 예를 들어 0x1234 레지스터에는 1과 2만 작성할 수 있다고 해보겠습니다.
            if register == 0x1234 {
                match data {
                    1 | 2 => {},
                    _ => {
                        // 에러를 반환합니다.
                        return Err(Error::Exception(Exception::IllegalDataValue));
                    }
                }
            }

            // 최종적으로 데이터를 레지스터에 적용합니다.
            let index: usize = holding_registers.find(register).unwrap();
            holding_registers.set(index, data);

            // 응답을 생성합니다.
            // * 또는 수신 버퍼 앞 8바이트를 복사하여 사용합니다.
            let response = Response::WriteSingleRegister(WriteSingleRegister::new(modbus_id, register, data));
            Ok(response)
        },

        Function::WriteMultipleRegisters { start, data } => {
            let len = data.len();

            // 유효한 주소인지 확인합니다.
            holding_registers.check_addresses(start, len)?;

            // 
        },
    };

    // 브로드캐스팅이면 응답 하지 않습니다.
    if is_broadcastring {
        return;
    }
}
```