import 'dart:io';
import 'dart:isolate';

import 'package:arche/arche.dart';
import 'package:cczu_ical_gui/native.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:url_launcher/url_launcher_string.dart';

void main(List<String> args) async {
  var bus = ArcheBus();
  bus.provide(ArcheConfig.path("app.config.json"));
  runApp(const MainApplication());
}

class MainApplication extends StatelessWidget {
  const MainApplication({super.key});

  @override
  Widget build(BuildContext context) {
    return PlatformProvider(
      initialPlatform: switch (ArcheBus.config.getOr("platformtheme", 0)) {
        1 => TargetPlatform.windows,
        2 => TargetPlatform.macOS,
        _ => null,
      },
      builder: (context) => PlatformTheme(
        themeMode: ThemeMode.system,
        materialDarkTheme: ThemeData.dark(useMaterial3: true),
        materialLightTheme: ThemeData.light(useMaterial3: true),
        builder: (context) => const PlatformApp(
          title: "常州大学课程日历生成器",
          home: HomePage(),
        ),
      ),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<StatefulWidget> createState() => StateHomePage();
}

class StateHomePage extends State<HomePage> {
  String pwd = "";
  String user = "";
  String reminder = "";

  static void _spawnical(SendPort sport) {
    ReceivePort rport = ReceivePort();
    sport.send(rport.sendPort);
    rport.listen((data) {
      if (data is List) {
        rport.close();
        sport.send(iCalJson(data[0], data[1], data[2], data[3]));
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return PlatformScaffold(
      appBar: PlatformAppBar(
        title: PlatformText("常州大学课程日历生成器"),
        trailingActions: [
          PlatformPopupMenu(options: [
            PopupMenuOption(
                label: "Windows",
                onTap: (_) => ArcheBus.config.write("platformtheme", 1)),
            PopupMenuOption(
                label: "MacOS",
                onTap: (_) => ArcheBus.config.write("platformtheme", 2)),
            PopupMenuOption(
                label: "Auto",
                onTap: (_) => ArcheBus.config.write("platformtheme", 0))
          ], icon: Icon(PlatformIcons(context).pen)),
          PlatformIconButton(
            icon: Icon(PlatformIcons(context).ellipsis),
            onPressed: () => showPlatformDialog(
              context: context,
              builder: (context) => Dialog.fullscreen(
                child: PlatformScaffold(
                  appBar: PlatformAppBar(
                    leading: PlatformIconButton(
                      icon: Icon(PlatformIcons(context).back),
                      onPressed: () => Navigator.pop(context),
                    ),
                  ),
                  body: ListView(
                    children: [
                      PlatformListTile(
                        leading: Icon(PlatformIcons(context).heartSolid),
                        title: PlatformText("开源地址"),
                        trailing: Icon(PlatformIcons(context).forward),
                        onTap: () => launchUrlString(
                            "https://github.com/CCZU-OSSA/CCZU-iCal-rs"),
                      ),
                      isMaterial(context)
                          ? PlatformListTile(
                              leading: Icon(PlatformIcons(context).book),
                              title: PlatformText("开源许可证"),
                              trailing: Icon(PlatformIcons(context).forward),
                              onTap: () => showAboutDialog(context: context),
                            )
                          : const SizedBox.shrink(),
                      PlatformListTile(
                        leading: Icon(PlatformIcons(context).home),
                        title: PlatformText("常州大学开源软件协会"),
                        trailing: Icon(PlatformIcons(context).forward),
                        onTap: () =>
                            launchUrlString("https://cczu-ossa.github.io/home"),
                      ),
                      PlatformListTile(
                        leading: Icon(PlatformIcons(context).help),
                        title: PlatformText("QQ群(947560153)"),
                        trailing: Icon(PlatformIcons(context).forward),
                        onTap: () => launchUrlString(
                            "http://qm.qq.com/cgi-bin/qm/qr?_wv=1027&k=6wgGLJ_NmKQl7f9Ws6JAprbTwmG9Ouei&authKey=g7bXX%2Bn2dHlbecf%2B8QfGJ15IFVOmEdGTJuoLYfviLg7TZIsZCu45sngzZfL3KktN&noverify=0&group_code=947560153"),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          )
        ],
      ),
      body: ListView(
        children: [
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              PlatformText(
                "学号",
                style: const TextStyle(fontSize: 24),
              ),
              SizedBox(
                width: 256,
                child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: PlatformTextField(
                      hintText: "2300000000",
                      onChanged: (p0) => user = p0,
                    )),
              )
            ],
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              PlatformText(
                "密码",
                style: const TextStyle(fontSize: 24),
              ),
              SizedBox(
                width: 256,
                child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: PlatformTextField(
                      hintText: "默认密码身份证后六位",
                      onChanged: (p0) => pwd = p0,
                      obscureText: true,
                    )),
              )
            ],
          ),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              PlatformText(
                "提醒",
                style: const TextStyle(fontSize: 24),
              ),
              SizedBox(
                width: 256,
                child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: PlatformTextField(
                      hintText: "15",
                      keyboardType: const TextInputType.numberWithOptions(
                          signed: true, decimal: false),
                      onChanged: (p0) => reminder = p0,
                    )),
              )
            ],
          ),
          Padding(
              padding: const EdgeInsets.fromLTRB(128, 32, 128, 0),
              child: PlatformElevatedButton(
                child: PlatformText("生成"),
                onPressed: () {
                  showPlatformDatePicker(
                    context: context,
                    initialDate: DateTime.now(),
                    firstDate:
                        DateTime.now().subtract(const Duration(days: 365)),
                    lastDate: DateTime.now(),
                  ).then((value) {
                    if (value != null) {
                      var date =
                          "${value.year}${value.month.toString().padLeft(2, "0")}${value.day.toString().padLeft(2, "0")}";
                      showPlatformDialog(
                        barrierDismissible: false,
                        context: context,
                        builder: (context) => Dialog.fullscreen(
                            backgroundColor: Colors.transparent,
                            child: Center(
                              child: PlatformCircularProgressIndicator(),
                            )),
                      );

                      ReceivePort rport = ReceivePort();
                      rport.listen((message) {
                        if (message is SendPort) {
                          message.send([user, pwd, date, reminder]);
                        } else {
                          ICalJsonData jdata = message;
                          rport.close();
                          if (jdata.ok) {
                            FilePicker.platform
                                .saveFile(fileName: "class.ics")
                                .then((value) {
                              if (value != null) {
                                File(value).writeAsStringSync(jdata.data);
                              }
                              Navigator.pop(context);
                            });
                          } else {
                            Navigator.pop(context);
                            showPlatformDialog(
                              context: context,
                              builder: (context) => PlatformAlertDialog(
                                content: PlatformText("😭错误，请检查用户名密码"),
                              ),
                            );
                          }
                        }
                      });
                      Isolate.spawn(_spawnical, rport.sendPort);
                    }
                  });
                },
              ))
        ],
      ),
    );
  }
}
