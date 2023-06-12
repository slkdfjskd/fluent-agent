import 'package:fluent/widget/environment/variableLis.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../common/colors.dart';
import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/environment/viewModel.dart';
import 'envList.dart';

Future<void> showEnvironmentDialog(BuildContext ctx) async {
  return showDialog(
      context: ctx,
      barrierDismissible: true,
      barrierColor: lightDialogBarrierColor,
      builder: (ctx) {
        Log.d('showImportProtoDialog build');
        return const Dialog(
          shape: RoundedRectangleBorder(
              side: BorderSide(color: rippleColor, width: 2),
              borderRadius: BorderRadius.all(Radius.circular(8))),
          child: EnvironmentDialog(),
        );
      });
}

class EnvironmentDialog extends StatelessWidget {
  const EnvironmentDialog({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    Log.d("EnvironmentDialog build");
    var viewModel = Provider.of<EnvironmentViewModel>(context);
    var children = [
      Stack(
        alignment: Alignment.center,
        children: [
          SizedBox(
              width: 210,
              child: Row(children: const [
                Text(
                  "Environment Variables",
                  style: TextStyle(
                      fontSize: fontSize,
                      color: blackFontColor,
                      fontWeight: FontWeight.bold),
                ),
              ])),
          Align(
              alignment: Alignment.centerLeft,
              child: IconButton(
                  onPressed: () {
                    Navigator.of(context).pop();
                  },
                  splashRadius: 13,
                  icon: const Icon(
                    Icons.close,
                    color: blackFontColor,
                    size: iconSize,
                  ))),
        ],
      ),
      EnvListWidget(viewModel: viewModel),
      Container(
          width: double.infinity,
          decoration: const BoxDecoration(
              border: Border(
                  top: BorderSide(color: rippleColor, width: 2),
                  bottom: BorderSide(color: rippleColor, width: 2))),
          height: 30,
          child: Row(
            mainAxisSize: MainAxisSize.max,
            children: [
              Container(
                  decoration: const BoxDecoration(
                      border: Border(
                          right: BorderSide(color: rippleColor, width: 2))),
                  width: 150,
                  alignment: Alignment.center,
                  child: const Text(
                    "Variable",
                    style: TextStyle(
                        fontSize: fontSize, fontWeight: FontWeight.bold),
                  )),
              Expanded(
                  child: Container(
                      alignment: Alignment.centerLeft,
                      padding: const EdgeInsets.only(left: 10),
                      child: const Text(
                        "Value",
                        style: TextStyle(
                            fontSize: fontSize, fontWeight: FontWeight.bold),
                      )))
            ],
          )),
      Expanded(child: VariableListWidget(viewModel: viewModel)),
      Container(
          height: 35,
          margin: const EdgeInsets.only(left: 5),
          child: Row(
            children: [
              IconButton(
                splashRadius: 13,
                icon: const Icon(Icons.add, size: iconSize + 2),
                onPressed: () async {
                  Log.d('add onPressed');
                  viewModel.createVariable();
                },
              ),
              IconButton(
                splashRadius: 13,
                icon: const Icon(Icons.remove, size: iconSize + 2),
                onPressed: () async {
                  var currentVar = viewModel.currentVariable();
                  if (currentVar == null) {
                    return;
                  }
                  bool? delete = await showDeleteConfirmDialog(
                      context,
                      'Delete one variable ?',
                      'Delete the variable ${currentVar.dto.name}');
                  Log.d('showDeleteConfirmDialog delete: $delete');
                  if (delete != null && delete) {
                    viewModel.deleteCurrentSelectedVariable();
                  }
                },
              )
            ],
          ))
    ];

    return Container(
        constraints: const BoxConstraints(minWidth: 600, minHeight: 500),
        width: 730,
        height: 550,
        child: Column(children: children));
  }
}

Future<bool?> showDeleteConfirmDialog(
    BuildContext ctx, String title, String content) {
  return showDialog<bool>(
    barrierColor: darkDialogBarrierColor,
    context: ctx,
    builder: (context) {
      return AlertDialog(
        title: Text(title, style: const TextStyle(fontWeight: FontWeight.bold)),
        titleTextStyle:
            const TextStyle(fontSize: fontSize + 2, color: Colors.black87),
        content: Text(content),
        contentTextStyle:
            const TextStyle(fontSize: fontSize, color: Colors.black87),
        actions: <Widget>[
          OutlinedButton(
            child: const Text("Cancel", style: TextStyle(fontSize: fontSize)),
            style: ButtonStyle(
                padding: MaterialStateProperty.all(const EdgeInsets.only(
                    left: 40, right: 40, top: 7, bottom: 7))),
            onPressed: () => Navigator.of(context).pop(), // 关闭对话框
          ),
          ElevatedButton(
            child: const Text("Delete", style: TextStyle(fontSize: fontSize)),
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
