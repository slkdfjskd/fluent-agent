import 'package:fluent/common/log.dart';
import 'package:fluent/common/sizes.dart' as sizes;
import 'package:fluent/model/bottomBar/viewModel.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../common/colors.dart' as colors;
import '../../common/colors.dart';
import '../../common/sizes.dart';

class BottomBarWidget extends StatelessWidget {
  const BottomBarWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var viewModel = Provider.of<BottomBarViewModel>(context);
    return Container(
        padding: const EdgeInsets.only(left: 5, right: 5),
        height: 30,
        color: Colors.white70,
        child: Stack(
          alignment:Alignment.center ,
          children: [
            Row(
              children: [
                TextButton.icon(
                    icon: const Icon(Icons.dvr,
                        color: colors.iconColor, size: iconSize),
                    label: const Text(
                      "Log",
                      style: TextStyle(
                          color: colors.blackFontColor,
                          fontSize: sizes.fontSize - 2),
                    ),
                    onPressed: () {
                      Log.d("onPressed");
                      viewModel.isOpenWithLog = !viewModel.isOpenWithLog;
                    }),
              ],
            ),
            Positioned(
              right: 5,
              child: TextButton.icon(
                icon: const Icon(Icons.comment,
                    color: colors.iconColor, size: iconSize),
                label: const Text(
                  "Feedback",
                  style: TextStyle(
                      color: colors.blackFontColor,
                      fontSize: sizes.fontSize - 2),
                ),
                onPressed: () {
                  Log.d("onPressed");
                  showFeedbackDialog(context);
                }),)
          ],
        ));
  }
}

Future<bool?> showFeedbackDialog(BuildContext ctx) {
  return showDialog<bool>(
    barrierColor: darkDialogBarrierColor,
    context: ctx,
    builder: (context) {
      return AlertDialog(
        title: const Text('Feedback',
            style: TextStyle(fontWeight: FontWeight.bold)),
        titleTextStyle:
        const TextStyle(fontSize: fontSize + 2, color: Colors.black87),
        content: const SelectableText("Issue: https://github.com/slkdfjskd/fluent-agent/issues\n\nEmail: fluentagent3721@gmail.com"),
        contentTextStyle:
        const TextStyle(fontSize: fontSize, color: Colors.black87),
        actions: <Widget>[
          ElevatedButton(
            child: const Text("Close", style: TextStyle(fontSize: fontSize)),
            style: ButtonStyle(
                padding: MaterialStateProperty.all(const EdgeInsets.only(
                    left: 40, right: 40, top: 7, bottom: 7))),
            onPressed: () {
              //关闭对话框并返回true
              Navigator.of(context).pop(true);
            },
          ),
        ],
        actionsAlignment: MainAxisAlignment.center,
        actionsPadding: const EdgeInsets.only(bottom: 5),
      );
    },
  );
}
