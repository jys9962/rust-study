/**
FnMut extends FnOnce
Fn extends FnMut

FnOnce -> FnMut -> Fn

FnOnce: 한 번 실행할 수 있음 > 소유권을 가져올 수 있음
FnMut: 캡처된 값을 변경할 수 있음
Fn: 캡처된 값이 없거나 변경하지 않음
**/


fn main() {}
