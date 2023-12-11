import 'dart:convert';
import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';

DynamicLibrary _loadnative() {
  if (Platform.isWindows) {
    return DynamicLibrary.open("natives/cczu-ical-rs-windows-amd64-lib.dll");
  } else if (Platform.isLinux) {
    return DynamicLibrary.open("natives/cczu-ical-rs-linux-amd64-lib.so");
  } else if (Platform.isMacOS) {
    return DynamicLibrary.open("cczu-ical-rs-macos-amd64-lib.dylib");
  }
  return DynamicLibrary.process();
}

DynamicLibrary _nativelib = _loadnative();

typedef _CString = Pointer<Utf8>;

_CString Function(_CString, _CString, _CString, _CString) _generateics =
    _nativelib
        .lookup<
            NativeFunction<
                _CString Function(_CString, _CString, _CString,
                    _CString)>>("generate_ics_safejson")
        .asFunction();

class ICalJsonData {
  final String data;
  final bool ok;
  ICalJsonData(this.data, this.ok);
}

ICalJsonData iCalJson(
    String username, String password, String firstweekday, String reminder) {
  var data = jsonDecode(_generateics(
          username.toNativeUtf8(),
          password.toNativeUtf8(),
          firstweekday.toNativeUtf8(),
          reminder.toNativeUtf8())
      .toDartString());
  return ICalJsonData(data["data"], data["ok"]);
}
