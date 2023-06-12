import 'package:flutter/material.dart';

import '../../common/colors.dart';
import '../../common/log.dart';
import '../../common/sizes.dart';
import '../../model/environment/environment.dart';
import '../../model/environment/viewModel.dart';
import 'manager.dart';

class EnvListWidget extends StatelessWidget {
  final EnvironmentViewModel viewModel;

  const EnvListWidget({Key? key, required this.viewModel}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    Log.d("EnvListWidget build");
    return Container(
        height: 30,
        decoration: const BoxDecoration(
            color: backgroundGreyColor,
            border: Border(bottom: BorderSide(color: Colors.grey, width: 0.5))),
        child: Material(
          color: backgroundGreyColor,
          child: Stack(children: [
            SizedBox(
                width: double.infinity,
                child: SingleChildScrollView(
                    padding: const EdgeInsets.only(right: 60),
                    scrollDirection: Axis.horizontal,
                    child: Row(
                        mainAxisAlignment: MainAxisAlignment.start,
                        children:
                            viewModel.envList.asMap().entries.map((entry) {
                          return _item(entry.key, entry.value);
                        }).toList()))),
            Positioned(
                right: 30,
                child: SizedBox(
                    width: 30,
                    height: 30,
                    child: IconButton(
                        onPressed: () {
                          viewModel.createEnv();
                        },
                        splashRadius: 13,
                        icon: const Icon(
                          Icons.add,
                          color: blackFontColor,
                          size: iconSize + 2,
                        )))),
            Positioned(
                right: 0,
                child: SizedBox(
                    width: 30,
                    height: 30,
                    child: IconButton(
                        onPressed: () async {
                          var env = viewModel.currentManagerEnv();
                          if (env == null) {
                            return;
                          }
                          bool? delete = await showDeleteConfirmDialog(
                              context,
                              'Delete one environment ?',
                              'Delete the environment ${env.envName}');
                          Log.d('showDeleteConfirmDialog delete: $delete');
                          if (delete != null && delete) {
                            viewModel.deleteCurrentSelectedEnv();
                          }
                        },
                        splashRadius: 13,
                        icon: const Icon(
                          Icons.remove,
                          color: blackFontColor,
                          size: iconSize + 2,
                        )))),
          ]),
        ));
  }

  Widget _item(int index, EnvironmentVO environmentVO) {
    Widget result;
    if (environmentVO.envNameEdit) {
      var controller = TextEditingController();
      controller.text = environmentVO.envName;
      controller.selection = TextSelection(
          baseOffset: 0, extentOffset: environmentVO.envName.length);
      FocusNode focusNode = FocusNode();
      focusNode.addListener(() {
        Log.d('focusNode Listener: ${focusNode.hasFocus}');
        if (!focusNode.hasFocus) {
          viewModel.editEnvName(index, false);
          viewModel.updateEnvName(index, controller.value.text);
          controller.dispose();
        }
      });
      result = Container(
          decoration: const BoxDecoration(
              border: Border(right: BorderSide(color: rippleColor, width: 2))),
          height: 30,
          width: 100,
          padding: const EdgeInsets.only(left: 3, right: 3, top: 2),
          child: TextField(
              textAlign: TextAlign.center,
              focusNode: focusNode,
              controller: controller,
              autofocus: true,
              maxLength: 64,
              style: const TextStyle(
                fontSize: fontSize,
              ),
              decoration: const InputDecoration(
                counterText: '',
                contentPadding: EdgeInsets.only(left: 3, top: 0, bottom: 0),
                border: OutlineInputBorder(
                    borderSide: BorderSide(width: 0.5),
                    borderRadius: BorderRadius.all(Radius.circular(3))),
              )));
    } else {
      result = InkWell(
          splashColor: rippleColor,
          onDoubleTap: () {
            Log.d('GestureDetector onDoubleTap');
            viewModel.editEnvName(index, true);
          },
          onTapDown: (TapDownDetails details) {
            Log.d('GestureDetector onTapDown');
            viewModel.selectManagerEnv(index);
          },
          child: Container(
              padding: const EdgeInsets.only(left: 5, right: 5),
              alignment: Alignment.center,
              decoration: const BoxDecoration(
                  border: Border(
                      top: BorderSide(color: rippleColor, width: 2),
                      right: BorderSide(color: rippleColor, width: 2))),
              height: 30,
              width: 100,
              child: Text(
                environmentVO.envName,
                overflow: TextOverflow.ellipsis,
                maxLines: 1,
                style: TextStyle(
                    fontSize: fontSize,
                    color: viewModel.envManagerSelectIndex == index
                        ? Colors.blue
                        : blackFontColor),
              )));
    }
    return result;
  }
}
